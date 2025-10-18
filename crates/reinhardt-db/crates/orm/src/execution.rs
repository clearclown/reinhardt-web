//! # Query Execution
//!
//! SQLAlchemy-inspired query execution methods.
//!
//! This module provides execution methods similar to SQLAlchemy's Query class

use crate::Model;
use std::marker::PhantomData;

/// Query execution result types
#[derive(Debug)]
pub enum ExecutionResult<T> {
    /// Single result
    One(T),
    /// Optional single result
    OneOrNone(Option<T>),
    /// Multiple results
    All(Vec<T>),
    /// Scalar value (for aggregates)
    Scalar(String),
    /// No result (for mutations)
    None,
}

/// Query execution methods
/// These would be async in a real implementation
pub trait QueryExecution<T: Model> {
    /// Get a single result by primary key
    /// Corresponds to SQLAlchemy's .get()
    fn get(&self, pk: &T::PrimaryKey) -> String;

    /// Get all results
    /// Corresponds to SQLAlchemy's .all()
    fn all(&self) -> String;

    /// Get first result or None
    /// Corresponds to SQLAlchemy's .first()
    fn first(&self) -> String;

    /// Get exactly one result, raise if 0 or >1
    /// Corresponds to SQLAlchemy's .one()
    fn one(&self) -> String;

    /// Get one result or None, raise if >1
    /// Corresponds to SQLAlchemy's .one_or_none()
    fn one_or_none(&self) -> String;

    /// Get scalar value (first column of first row)
    /// Corresponds to SQLAlchemy's .scalar()
    fn scalar(&self) -> String;

    /// Count results
    /// Corresponds to SQLAlchemy's .count()
    fn count(&self) -> String;

    /// Check if any results exist
    /// Corresponds to SQLAlchemy's .exists()
    fn exists(&self) -> String;
}

/// Execution context for SELECT queries
pub struct SelectExecution<T: Model> {
    sql: String,
    _phantom: PhantomData<T>,
}

impl<T: Model> SelectExecution<T> {
    /// Create a new query execution context with the given SQL
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_orm::execution::SelectExecution;
    /// use reinhardt_orm::Model;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// struct User {
    ///     id: Option<i64>,
    ///     name: String,
    /// }
    ///
    /// impl Model for User {
    ///     type PrimaryKey = i64;
    ///     fn table_name() -> &'static str { "users" }
    ///     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
    ///     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
    /// }
    ///
    /// let exec = SelectExecution::<User>::new("SELECT * FROM users".to_string());
    /// assert_eq!(exec.sql(), "SELECT * FROM users");
    /// ```
    pub fn new(sql: String) -> Self {
        Self {
            sql,
            _phantom: PhantomData,
        }
    }
    /// Get the underlying SQL query
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_orm::execution::SelectExecution;
    /// use reinhardt_orm::Model;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// struct User {
    ///     id: Option<i64>,
    ///     name: String,
    /// }
    ///
    /// impl Model for User {
    ///     type PrimaryKey = i64;
    ///     fn table_name() -> &'static str { "users" }
    ///     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
    ///     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
    /// }
    ///
    /// let exec = SelectExecution::<User>::new("SELECT * FROM users WHERE active = true".to_string());
    /// assert!(exec.sql().contains("WHERE"));
    /// ```
    pub fn sql(&self) -> &str {
        &self.sql
    }
}

impl<T: Model> QueryExecution<T> for SelectExecution<T>
where
    T::PrimaryKey: std::fmt::Debug,
{
    fn get(&self, pk: &T::PrimaryKey) -> String {
        format!(
            "SELECT * FROM {} WHERE {} = '{:?}' LIMIT 1",
            T::table_name(),
            T::primary_key_field(),
            pk
        )
    }

    fn all(&self) -> String {
        self.sql.clone()
    }

    fn first(&self) -> String {
        if self.sql.contains("LIMIT") {
            self.sql.clone()
        } else {
            format!("{} LIMIT 1", self.sql)
        }
    }

    fn one(&self) -> String {
        // In real implementation, this would check result count
        format!("{} LIMIT 2 /* EXPECT ONE */", self.sql)
    }

    fn one_or_none(&self) -> String {
        format!("{} LIMIT 2 /* EXPECT ZERO OR ONE */", self.sql)
    }

    fn scalar(&self) -> String {
        format!("{} LIMIT 1 /* SCALAR */", self.sql)
    }

    fn count(&self) -> String {
        // Extract FROM clause and WHERE clause from original SQL
        if self.sql.contains("FROM") {
            let parts: Vec<&str> = self.sql.split("FROM").collect();
            if parts.len() > 1 {
                return format!("SELECT COUNT(*) FROM {}", parts[1]);
            }
        }
        format!("SELECT COUNT(*) FROM {}", T::table_name())
    }

    fn exists(&self) -> String {
        format!("SELECT EXISTS({})", self.sql)
    }
}

/// Loading options for relationships
/// Corresponds to SQLAlchemy's loader options
#[derive(Debug, Clone)]
pub enum LoadOption {
    /// Eager load with JOIN
    /// Corresponds to joinedload()
    JoinedLoad(String),

    /// Eager load with separate SELECT
    /// Corresponds to selectinload()
    SelectInLoad(String),

    /// Lazy load on access
    /// Corresponds to lazyload()
    LazyLoad(String),

    /// Don't load at all
    /// Corresponds to noload()
    NoLoad(String),

    /// Raise error if accessed
    /// Corresponds to raiseload()
    RaiseLoad(String),

    /// Defer column loading
    /// Corresponds to defer()
    Defer(String),

    /// Undefer column loading
    /// Corresponds to undefer()
    Undefer(String),

    /// Load only specified columns
    /// Corresponds to load_only()
    LoadOnly(Vec<String>),
}

impl LoadOption {
    /// Convert load option to SQL comment for debugging
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_orm::execution::LoadOption;
    ///
    /// let option = LoadOption::JoinedLoad("profile".to_string());
    /// assert_eq!(option.to_sql_comment(), "/* joinedload(profile) */");
    ///
    /// let option = LoadOption::Defer("password".to_string());
    /// assert_eq!(option.to_sql_comment(), "/* defer(password) */");
    ///
    /// let option = LoadOption::LoadOnly(vec!["id".to_string(), "name".to_string()]);
    /// assert_eq!(option.to_sql_comment(), "/* load_only(id, name) */");
    /// ```
    pub fn to_sql_comment(&self) -> String {
        match self {
            LoadOption::JoinedLoad(rel) => format!("/* joinedload({}) */", rel),
            LoadOption::SelectInLoad(rel) => format!("/* selectinload({}) */", rel),
            LoadOption::LazyLoad(rel) => format!("/* lazyload({}) */", rel),
            LoadOption::NoLoad(rel) => format!("/* noload({}) */", rel),
            LoadOption::RaiseLoad(rel) => format!("/* raiseload({}) */", rel),
            LoadOption::Defer(col) => format!("/* defer({}) */", col),
            LoadOption::Undefer(col) => format!("/* undefer({}) */", col),
            LoadOption::LoadOnly(cols) => format!("/* load_only({}) */", cols.join(", ")),
        }
    }
}

/// Query options container
pub struct QueryOptions {
    pub load_options: Vec<LoadOption>,
}

impl QueryOptions {
    /// Create a new empty query options container
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_orm::execution::QueryOptions;
    ///
    /// let options = QueryOptions::new();
    /// assert_eq!(options.to_sql_comments(), "");
    /// ```
    pub fn new() -> Self {
        Self {
            load_options: Vec::new(),
        }
    }
    /// Add a load option to the query
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_orm::execution::{QueryOptions, LoadOption};
    ///
    /// let options = QueryOptions::new()
    ///     .add_option(LoadOption::JoinedLoad("profile".to_string()))
    ///     .add_option(LoadOption::Defer("password".to_string()));
    ///
    /// let comments = options.to_sql_comments();
    /// assert!(comments.contains("joinedload(profile)"));
    /// assert!(comments.contains("defer(password)"));
    /// ```
    pub fn add_option(mut self, option: LoadOption) -> Self {
        self.load_options.push(option);
        self
    }
    /// Convert all options to SQL comments
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_orm::execution::{QueryOptions, LoadOption};
    ///
    /// let options = QueryOptions::new()
    ///     .add_option(LoadOption::SelectInLoad("posts".to_string()));
    ///
    /// assert!(options.to_sql_comments().contains("selectinload(posts)"));
    /// ```
    pub fn to_sql_comments(&self) -> String {
        if self.load_options.is_empty() {
            String::new()
        } else {
            format!(
                " {}",
                self.load_options
                    .iter()
                    .map(|o| o.to_sql_comment())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        }
    }
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reinhardt_validators::TableName;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct User {
        id: Option<i64>,
        name: String,
    }

    const USER_TABLE: TableName = TableName::new_const("users");

    impl Model for User {
        type PrimaryKey = i64;

        fn table_name() -> &'static str {
            USER_TABLE.as_str()
        }

        fn primary_key(&self) -> Option<&Self::PrimaryKey> {
            self.id.as_ref()
        }

        fn set_primary_key(&mut self, value: Self::PrimaryKey) {
            self.id = Some(value);
        }
    }

    #[test]
    fn test_execution_get() {
        let exec = SelectExecution::<User>::new("SELECT * FROM users".to_string());
        let sql = exec.get(&123);
        assert!(sql.contains("WHERE"));
        assert!(sql.contains("123"));
        assert!(sql.contains("LIMIT 1"));
    }

    #[test]
    fn test_all() {
        let exec = SelectExecution::<User>::new("SELECT * FROM users".to_string());
        let sql = exec.all();
        assert_eq!(sql, "SELECT * FROM users");
    }

    #[test]
    fn test_first() {
        let exec =
            SelectExecution::<User>::new("SELECT * FROM users WHERE active = true".to_string());
        let sql = exec.first();
        assert!(sql.contains("LIMIT 1"));
    }

    #[test]
    fn test_execution_count() {
        let exec =
            SelectExecution::<User>::new("SELECT * FROM users WHERE active = true".to_string());
        let sql = exec.count();
        assert!(sql.contains("COUNT(*)"));
    }

    #[test]
    fn test_execution_exists() {
        let exec =
            SelectExecution::<User>::new("SELECT * FROM users WHERE name = 'Alice'".to_string());
        let sql = exec.exists();
        assert!(sql.contains("EXISTS"));
    }

    #[test]
    fn test_load_options() {
        let options = QueryOptions::new()
            .add_option(LoadOption::JoinedLoad("profile".to_string()))
            .add_option(LoadOption::Defer("password".to_string()));

        let comments = options.to_sql_comments();
        assert!(comments.contains("joinedload(profile)"));
        assert!(comments.contains("defer(password)"));
    }

    #[test]
    fn test_load_only() {
        let option = LoadOption::LoadOnly(vec!["id".to_string(), "name".to_string()]);
        let comment = option.to_sql_comment();
        assert!(comment.contains("load_only(id, name)"));
    }
}
