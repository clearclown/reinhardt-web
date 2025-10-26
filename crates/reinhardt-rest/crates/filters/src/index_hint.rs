//! Database index hint system for query optimization
//!
//! Provides intelligent index usage hints to optimize database query performance.
//!
//! # Examples
//!
//! ```
//! use reinhardt_filters::{FilterBackend, IndexHintFilter, IndexStrategy};
//! use std::collections::HashMap;
//!
//! # async fn example() {
//! // Create filter with index hints
//! let filter = IndexHintFilter::new()
//!     .with_index("idx_users_email", IndexStrategy::Use)
//!     .with_index("idx_users_created_at", IndexStrategy::Force);
//!
//! let params = HashMap::new();
//! let sql = "SELECT * FROM users".to_string();
//! let result = filter.filter_queryset(&params, sql).await;
//! # }
//! ```

use crate::filter::{FilterBackend, FilterResult};
use async_trait::async_trait;
use std::collections::HashMap;

/// Index usage strategy
///
/// Specifies how the database should use the suggested index.
///
/// # Examples
///
/// ```
/// use reinhardt_filters::IndexStrategy;
///
/// let strategy = IndexStrategy::Use;
/// let force_strategy = IndexStrategy::Force;
/// let ignore_strategy = IndexStrategy::Ignore;
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexStrategy {
    /// Suggest using the index (USE INDEX hint)
    ///
    /// The database optimizer may choose to use the index or not.
    Use,

    /// Force using the index (FORCE INDEX hint)
    ///
    /// The database optimizer will strongly prefer this index.
    Force,

    /// Ignore the index (IGNORE INDEX hint)
    ///
    /// The database optimizer will not use this index.
    Ignore,
}

/// Configuration for an index hint
///
/// # Examples
///
/// ```
/// use reinhardt_filters::{IndexHint, IndexStrategy};
///
/// let hint = IndexHint::new("idx_users_email", IndexStrategy::Use);
/// ```
#[derive(Debug, Clone)]
pub struct IndexHint {
    /// Name of the index
    pub index_name: String,

    /// Strategy for using the index
    pub strategy: IndexStrategy,

    /// Table name (optional, for multi-table queries)
    pub table_name: Option<String>,
}

impl IndexHint {
    /// Create a new index hint
    ///
    /// # Arguments
    ///
    /// * `index_name` - Name of the database index
    /// * `strategy` - Strategy for using the index
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_filters::{IndexHint, IndexStrategy};
    ///
    /// let hint = IndexHint::new("idx_users_email", IndexStrategy::Use);
    /// ```
    pub fn new(index_name: impl Into<String>, strategy: IndexStrategy) -> Self {
        Self {
            index_name: index_name.into(),
            strategy,
            table_name: None,
        }
    }

    /// Specify the table name for this index hint
    ///
    /// Useful for multi-table queries where index names might be ambiguous.
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_filters::{IndexHint, IndexStrategy};
    ///
    /// let hint = IndexHint::new("idx_email", IndexStrategy::Use)
    ///     .for_table("users");
    /// ```
    pub fn for_table(mut self, table_name: impl Into<String>) -> Self {
        self.table_name = Some(table_name.into());
        self
    }

    /// Generate SQL hint clause for this index
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_filters::{IndexHint, IndexStrategy};
    ///
    /// let hint = IndexHint::new("idx_users_email", IndexStrategy::Use);
    /// let sql = hint.to_sql_hint();
    /// assert!(sql.contains("USE INDEX"));
    /// ```
    pub fn to_sql_hint(&self) -> String {
        let hint_type = match self.strategy {
            IndexStrategy::Use => "USE INDEX",
            IndexStrategy::Force => "FORCE INDEX",
            IndexStrategy::Ignore => "IGNORE INDEX",
        };

        format!("{} ({})", hint_type, self.index_name)
    }
}

/// Filter backend that adds database index hints to optimize query performance
///
/// This filter helps optimize database queries by suggesting which indexes
/// the query planner should use, force, or ignore.
///
/// # Database Compatibility
///
/// Currently supports MySQL/MariaDB syntax. PostgreSQL and other databases
/// use different hint mechanisms.
///
/// # Examples
///
/// ```
/// use reinhardt_filters::{FilterBackend, IndexHintFilter, IndexStrategy};
/// use std::collections::HashMap;
///
/// # async fn example() {
/// let filter = IndexHintFilter::new()
///     .with_index("idx_users_email", IndexStrategy::Use)
///     .with_index("idx_users_created_at", IndexStrategy::Force);
///
/// let params = HashMap::new();
/// let sql = "SELECT * FROM users".to_string();
/// let result = filter.filter_queryset(&params, sql).await;
/// # }
/// ```
#[derive(Debug, Default)]
pub struct IndexHintFilter {
    hints: Vec<IndexHint>,
    enabled: bool,
}

impl IndexHintFilter {
    /// Create a new index hint filter
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_filters::IndexHintFilter;
    ///
    /// let filter = IndexHintFilter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            hints: Vec::new(),
            enabled: true,
        }
    }

    /// Add an index hint to the filter
    ///
    /// # Arguments
    ///
    /// * `index_name` - Name of the database index
    /// * `strategy` - Strategy for using the index
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_filters::{IndexHintFilter, IndexStrategy};
    ///
    /// let filter = IndexHintFilter::new()
    ///     .with_index("idx_users_email", IndexStrategy::Use)
    ///     .with_index("idx_users_created_at", IndexStrategy::Force);
    /// ```
    pub fn with_index(mut self, index_name: impl Into<String>, strategy: IndexStrategy) -> Self {
        self.hints.push(IndexHint::new(index_name, strategy));
        self
    }

    /// Add a custom index hint
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_filters::{IndexHintFilter, IndexHint, IndexStrategy};
    ///
    /// let hint = IndexHint::new("idx_email", IndexStrategy::Use)
    ///     .for_table("users");
    ///
    /// let filter = IndexHintFilter::new()
    ///     .with_hint(hint);
    /// ```
    pub fn with_hint(mut self, hint: IndexHint) -> Self {
        self.hints.push(hint);
        self
    }

    /// Enable or disable index hints
    ///
    /// When disabled, hints are not applied to queries.
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_filters::{IndexHintFilter, IndexStrategy};
    ///
    /// let filter = IndexHintFilter::new()
    ///     .with_index("idx_users_email", IndexStrategy::Use)
    ///     .set_enabled(false);  // Temporarily disable hints
    /// ```
    pub fn set_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Apply index hints to SQL query
    ///
    /// Note: This is a placeholder implementation. Full implementation requires
    /// SQL parsing and table detection.
    fn apply_hints(&self, sql: String) -> FilterResult<String> {
        if !self.enabled || self.hints.is_empty() {
            return Ok(sql);
        }

        // TODO: Implement proper SQL parsing and hint injection
        // Current implementation is a basic placeholder
        todo!(
            "Implement SQL parsing to inject index hints at appropriate locations. \
             This requires identifying table references and inserting hints after table names."
        )
    }
}

#[async_trait]
impl FilterBackend for IndexHintFilter {
    async fn filter_queryset(
        &self,
        _query_params: &HashMap<String, String>,
        sql: String,
    ) -> FilterResult<String> {
        self.apply_hints(sql)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_strategy_variants() {
        let strategies = vec![
            IndexStrategy::Use,
            IndexStrategy::Force,
            IndexStrategy::Ignore,
        ];
        assert_eq!(strategies.len(), 3);
    }

    #[test]
    fn test_index_hint_creation() {
        let hint = IndexHint::new("idx_users_email", IndexStrategy::Use);
        assert_eq!(hint.index_name, "idx_users_email");
        assert_eq!(hint.strategy, IndexStrategy::Use);
        assert!(hint.table_name.is_none());
    }

    #[test]
    fn test_index_hint_with_table() {
        let hint = IndexHint::new("idx_email", IndexStrategy::Force).for_table("users");
        assert_eq!(hint.index_name, "idx_email");
        assert_eq!(hint.strategy, IndexStrategy::Force);
        assert_eq!(hint.table_name, Some("users".to_string()));
    }

    #[test]
    fn test_index_hint_to_sql_use() {
        let hint = IndexHint::new("idx_users_email", IndexStrategy::Use);
        let sql = hint.to_sql_hint();
        assert_eq!(sql, "USE INDEX (idx_users_email)");
    }

    #[test]
    fn test_index_hint_to_sql_force() {
        let hint = IndexHint::new("idx_users_created_at", IndexStrategy::Force);
        let sql = hint.to_sql_hint();
        assert_eq!(sql, "FORCE INDEX (idx_users_created_at)");
    }

    #[test]
    fn test_index_hint_to_sql_ignore() {
        let hint = IndexHint::new("idx_users_status", IndexStrategy::Ignore);
        let sql = hint.to_sql_hint();
        assert_eq!(sql, "IGNORE INDEX (idx_users_status)");
    }

    #[test]
    fn test_index_hint_filter_creation() {
        let filter = IndexHintFilter::new();
        assert!(filter.hints.is_empty());
        assert!(filter.enabled);
    }

    #[test]
    fn test_index_hint_filter_with_hints() {
        let filter = IndexHintFilter::new()
            .with_index("idx_users_email", IndexStrategy::Use)
            .with_index("idx_users_created_at", IndexStrategy::Force);

        assert_eq!(filter.hints.len(), 2);
        assert_eq!(filter.hints[0].index_name, "idx_users_email");
        assert_eq!(filter.hints[1].index_name, "idx_users_created_at");
    }

    #[test]
    fn test_index_hint_filter_disable() {
        let filter = IndexHintFilter::new()
            .with_index("idx_users_email", IndexStrategy::Use)
            .set_enabled(false);

        assert!(!filter.enabled);
    }

    #[tokio::test]
    async fn test_index_hint_filter_disabled_passthrough() {
        let filter = IndexHintFilter::new()
            .with_index("idx_users_email", IndexStrategy::Use)
            .set_enabled(false);

        let params = HashMap::new();
        let sql = "SELECT * FROM users".to_string();
        let result = filter.filter_queryset(&params, sql.clone()).await.unwrap();

        assert_eq!(result, sql);
    }

    #[tokio::test]
    async fn test_index_hint_filter_no_hints_passthrough() {
        let filter = IndexHintFilter::new();

        let params = HashMap::new();
        let sql = "SELECT * FROM users".to_string();
        let result = filter.filter_queryset(&params, sql.clone()).await.unwrap();

        assert_eq!(result, sql);
    }
}
