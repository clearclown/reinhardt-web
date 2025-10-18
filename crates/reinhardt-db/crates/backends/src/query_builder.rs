//! Query builder with dialect support

use std::sync::Arc;

use crate::{
    backend::DatabaseBackend,
    error::Result,
    types::{QueryResult, QueryValue, Row},
};

/// INSERT query builder
pub struct InsertBuilder {
    backend: Arc<dyn DatabaseBackend>,
    table: String,
    columns: Vec<String>,
    values: Vec<QueryValue>,
    returning: Option<Vec<String>>,
}

impl InsertBuilder {
    pub fn new(backend: Arc<dyn DatabaseBackend>, table: impl Into<String>) -> Self {
        Self {
            backend,
            table: table.into(),
            columns: Vec::new(),
            values: Vec::new(),
            returning: None,
        }
    }

    pub fn value(mut self, column: impl Into<String>, value: impl Into<QueryValue>) -> Self {
        self.columns.push(column.into());
        self.values.push(value.into());
        self
    }

    pub fn returning(mut self, columns: Vec<&str>) -> Self {
        if self.backend.supports_returning() {
            self.returning = Some(columns.iter().map(|s| s.to_string()).collect());
        }
        self
    }

    pub fn build(&self) -> (String, Vec<QueryValue>) {
        let mut sql = format!("INSERT INTO {} (", self.table);
        sql.push_str(&self.columns.join(", "));
        sql.push_str(") VALUES (");

        let placeholders: Vec<String> = (0..self.columns.len())
            .map(|i| self.backend.placeholder(i + 1))
            .collect();
        sql.push_str(&placeholders.join(", "));
        sql.push(')');

        if let Some(ref cols) = self.returning {
            sql.push_str(" RETURNING ");
            sql.push_str(&cols.join(", "));
        }

        (sql, self.values.clone())
    }

    pub async fn execute(&self) -> Result<QueryResult> {
        let (sql, params) = self.build();
        self.backend.execute(&sql, params).await
    }

    pub async fn fetch_one(&self) -> Result<Row> {
        let (sql, params) = self.build();
        self.backend.fetch_one(&sql, params).await
    }
}

/// UPDATE query builder
pub struct UpdateBuilder {
    backend: Arc<dyn DatabaseBackend>,
    table: String,
    sets: Vec<(String, QueryValue)>,
    wheres: Vec<(String, String, QueryValue)>,
}

impl UpdateBuilder {
    pub fn new(backend: Arc<dyn DatabaseBackend>, table: impl Into<String>) -> Self {
        Self {
            backend,
            table: table.into(),
            sets: Vec::new(),
            wheres: Vec::new(),
        }
    }

    pub fn set(mut self, column: impl Into<String>, value: impl Into<QueryValue>) -> Self {
        self.sets.push((column.into(), value.into()));
        self
    }

    pub fn set_now(mut self, column: impl Into<String>) -> Self {
        self.sets
            .push((column.into(), QueryValue::String("__NOW__".to_string())));
        self
    }

    pub fn where_eq(mut self, column: impl Into<String>, value: impl Into<QueryValue>) -> Self {
        self.wheres
            .push((column.into(), "=".to_string(), value.into()));
        self
    }

    pub fn build(&self) -> (String, Vec<QueryValue>) {
        let mut sql = format!("UPDATE {} SET ", self.table);
        let mut params = Vec::new();
        let mut param_index = 1;

        let set_clauses: Vec<String> = self
            .sets
            .iter()
            .map(|(col, val)| {
                if let QueryValue::String(s) = val {
                    if s == "__NOW__" {
                        return format!("{} = NOW()", col);
                    }
                }
                params.push(val.clone());
                let placeholder = self.backend.placeholder(param_index);
                param_index += 1;
                format!("{} = {}", col, placeholder)
            })
            .collect();

        sql.push_str(&set_clauses.join(", "));

        if !self.wheres.is_empty() {
            sql.push_str(" WHERE ");
            let where_clauses: Vec<String> = self
                .wheres
                .iter()
                .map(|(col, op, val)| {
                    params.push(val.clone());
                    let placeholder = self.backend.placeholder(param_index);
                    param_index += 1;
                    format!("{} {} {}", col, op, placeholder)
                })
                .collect();
            sql.push_str(&where_clauses.join(" AND "));
        }

        (sql, params)
    }

    pub async fn execute(&self) -> Result<QueryResult> {
        let (sql, params) = self.build();
        self.backend.execute(&sql, params).await
    }
}

/// SELECT query builder
pub struct SelectBuilder {
    backend: Arc<dyn DatabaseBackend>,
    columns: Vec<String>,
    table: String,
    wheres: Vec<(String, String, QueryValue)>,
    limit: Option<i64>,
}

impl SelectBuilder {
    pub fn new(backend: Arc<dyn DatabaseBackend>) -> Self {
        Self {
            backend,
            columns: vec!["*".to_string()],
            table: String::new(),
            wheres: Vec::new(),
            limit: None,
        }
    }

    pub fn columns(mut self, columns: Vec<&str>) -> Self {
        self.columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn from(mut self, table: impl Into<String>) -> Self {
        self.table = table.into();
        self
    }

    pub fn where_eq(mut self, column: impl Into<String>, value: impl Into<QueryValue>) -> Self {
        self.wheres
            .push((column.into(), "=".to_string(), value.into()));
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn build(&self) -> (String, Vec<QueryValue>) {
        let mut sql = format!("SELECT {} FROM {}", self.columns.join(", "), self.table);
        let mut params = Vec::new();
        let mut param_index = 1;

        if !self.wheres.is_empty() {
            sql.push_str(" WHERE ");
            let where_clauses: Vec<String> = self
                .wheres
                .iter()
                .map(|(col, op, val)| {
                    params.push(val.clone());
                    let placeholder = self.backend.placeholder(param_index);
                    param_index += 1;
                    format!("{} {} {}", col, op, placeholder)
                })
                .collect();
            sql.push_str(&where_clauses.join(" AND "));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        (sql, params)
    }

    pub async fn fetch_all(&self) -> Result<Vec<Row>> {
        let (sql, params) = self.build();
        self.backend.fetch_all(&sql, params).await
    }

    pub async fn fetch_one(&self) -> Result<Row> {
        let (sql, params) = self.build();
        self.backend.fetch_one(&sql, params).await
    }
}

/// DELETE query builder
pub struct DeleteBuilder {
    backend: Arc<dyn DatabaseBackend>,
    table: String,
    wheres: Vec<(String, String, QueryValue)>,
}

impl DeleteBuilder {
    pub fn new(backend: Arc<dyn DatabaseBackend>, table: impl Into<String>) -> Self {
        Self {
            backend,
            table: table.into(),
            wheres: Vec::new(),
        }
    }

    pub fn where_eq(mut self, column: impl Into<String>, value: impl Into<QueryValue>) -> Self {
        self.wheres
            .push((column.into(), "=".to_string(), value.into()));
        self
    }

    pub fn where_in(mut self, column: impl Into<String> + Clone, values: Vec<QueryValue>) -> Self {
        for value in values {
            self.wheres
                .push((column.clone().into(), "IN".to_string(), value));
        }
        self
    }

    pub fn build(&self) -> (String, Vec<QueryValue>) {
        let mut sql = format!("DELETE FROM {}", self.table);
        let mut params = Vec::new();
        let mut param_index = 1;

        if !self.wheres.is_empty() {
            sql.push_str(" WHERE ");
            let where_clauses: Vec<String> = self
                .wheres
                .iter()
                .map(|(col, op, val)| {
                    params.push(val.clone());
                    let placeholder = self.backend.placeholder(param_index);
                    param_index += 1;
                    format!("{} {} {}", col, op, placeholder)
                })
                .collect();
            sql.push_str(&where_clauses.join(" AND "));
        }

        (sql, params)
    }

    pub async fn execute(&self) -> Result<QueryResult> {
        let (sql, params) = self.build();
        self.backend.execute(&sql, params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::DatabaseBackend;
    use crate::types::{DatabaseType, QueryResult, QueryValue, Row};

    struct MockBackend;

    #[async_trait::async_trait]
    impl DatabaseBackend for MockBackend {
        fn database_type(&self) -> DatabaseType {
            DatabaseType::Postgres
        }

        fn placeholder(&self, index: usize) -> String {
            format!("${}", index)
        }

        fn supports_returning(&self) -> bool {
            true
        }

        fn supports_on_conflict(&self) -> bool {
            true
        }

        async fn execute(&self, _sql: &str, _params: Vec<QueryValue>) -> Result<QueryResult> {
            Ok(QueryResult { rows_affected: 1 })
        }

        async fn fetch_one(&self, _sql: &str, _params: Vec<QueryValue>) -> Result<Row> {
            Ok(Row::new())
        }

        async fn fetch_all(&self, _sql: &str, _params: Vec<QueryValue>) -> Result<Vec<Row>> {
            Ok(Vec::new())
        }

        async fn fetch_optional(
            &self,
            _sql: &str,
            _params: Vec<QueryValue>,
        ) -> Result<Option<Row>> {
            Ok(None)
        }
    }

    #[test]
    fn test_delete_builder_basic() {
        let backend = Arc::new(MockBackend);
        let builder = DeleteBuilder::new(backend, "users");
        let (sql, params) = builder.build();

        assert_eq!(sql, "DELETE FROM users");
        assert!(params.is_empty());
    }

    #[test]
    fn test_delete_builder_where_eq() {
        let backend = Arc::new(MockBackend);
        let builder = DeleteBuilder::new(backend, "users").where_eq("id", QueryValue::Int(1));
        let (sql, params) = builder.build();

        assert_eq!(sql, "DELETE FROM users WHERE id = $1");
        assert_eq!(params.len(), 1);
        assert!(matches!(params[0], QueryValue::Int(1)));
    }

    #[test]
    fn test_delete_builder_where_in() {
        let backend = Arc::new(MockBackend);
        let builder = DeleteBuilder::new(backend, "users")
            .where_in("id", vec![QueryValue::Int(1), QueryValue::Int(2)]);
        let (sql, params) = builder.build();

        assert_eq!(sql, "DELETE FROM users WHERE id IN $1 AND id IN $2");
        assert_eq!(params.len(), 2);
        assert!(matches!(params[0], QueryValue::Int(1)));
        assert!(matches!(params[1], QueryValue::Int(2)));
    }

    #[test]
    fn test_delete_builder_multiple_conditions() {
        let backend = Arc::new(MockBackend);
        let builder = DeleteBuilder::new(backend, "users")
            .where_eq("status", QueryValue::String("inactive".to_string()))
            .where_eq("age", QueryValue::Int(18));
        let (sql, params) = builder.build();

        assert_eq!(sql, "DELETE FROM users WHERE status = $1 AND age = $2");
        assert_eq!(params.len(), 2);
    }
}
