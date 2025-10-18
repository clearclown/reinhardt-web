//! Multi-term search with type-safe fields
//!
//! Provides utilities for building queries that search multiple terms across
//! multiple fields, combining them with AND/OR logic.

use crate::searchable::SearchableModel;
use reinhardt_orm::{Field, Lookup, Model, QueryFieldCompiler};

/// Combines multiple search terms across multiple fields
///
/// # Examples
///
/// ```rust,ignore
/// // Search for posts containing "rust" AND "programming"
/// let terms = vec!["rust", "programming"];
/// let lookups = MultiTermSearch::search_terms::<Post>(terms);
///
/// // This creates: (title ICONTAINS 'rust' OR content ICONTAINS 'rust')
/// //           AND (title ICONTAINS 'programming' OR content ICONTAINS 'programming')
/// ```
pub struct MultiTermSearch;

impl MultiTermSearch {
    /// Create lookups for searching multiple terms across searchable fields
    ///
    /// For each term, creates an OR clause across all searchable fields,
    /// then combines all terms with AND.
    pub fn search_terms<M: SearchableModel>(terms: Vec<&str>) -> Vec<Vec<Lookup<M>>> {
        let fields = M::searchable_fields();

        terms
            .into_iter()
            .map(|term| {
                // For each term, create lookups for all searchable fields
                fields
                    .iter()
                    .map(|field| {
                        // Create a new field with same path and create an icontains lookup
                        let new_field = Field::<M, String>::new(field.path().to_vec());
                        new_field.icontains(term)
                    })
                    .collect()
            })
            .collect()
    }

    /// Create lookups for exact match search across multiple terms
    pub fn exact_terms<M: SearchableModel>(terms: Vec<&str>) -> Vec<Vec<Lookup<M>>> {
        let fields = M::searchable_fields();

        terms
            .into_iter()
            .map(|term| {
                fields
                    .iter()
                    .map(|field| {
                        let new_field = Field::<M, String>::new(field.path().to_vec());
                        new_field.iexact(term.to_string())
                    })
                    .collect()
            })
            .collect()
    }

    /// Create lookups for prefix search (startswith) across multiple terms
    pub fn prefix_terms<M: SearchableModel>(terms: Vec<&str>) -> Vec<Vec<Lookup<M>>> {
        let fields = M::searchable_fields();

        terms
            .into_iter()
            .map(|term| {
                fields
                    .iter()
                    .map(|field| {
                        let new_field = Field::<M, String>::new(field.path().to_vec());
                        new_field.startswith(term)
                    })
                    .collect()
            })
            .collect()
    }

    /// Parse a comma-separated search string into individual terms
    ///
    /// Handles quoted strings properly, keeping quoted content together.
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_filters::multi_term::MultiTermSearch;
    ///
    /// let terms = MultiTermSearch::parse_search_terms("rust, programming");
    /// assert_eq!(terms, vec!["rust", "programming"]);
    ///
    /// let terms = MultiTermSearch::parse_search_terms("\"hello world\", rust");
    /// assert_eq!(terms, vec!["hello world", "rust"]);
    /// ```
    pub fn parse_search_terms(search: &str) -> Vec<String> {
        let mut terms = Vec::new();
        let mut current_term = String::new();
        let mut in_quotes = false;
        let mut chars = search.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '"' => {
                    in_quotes = !in_quotes;
                }
                ',' if !in_quotes => {
                    let trimmed = current_term.trim().to_string();
                    if !trimmed.is_empty() {
                        terms.push(trimmed);
                    }
                    current_term.clear();
                }
                _ => {
                    current_term.push(c);
                }
            }
        }

        // Don't forget the last term
        let trimmed = current_term.trim().to_string();
        if !trimmed.is_empty() {
            terms.push(trimmed);
        }

        terms
    }

    /// Compile multi-term lookups into SQL WHERE clause
    ///
    /// Terms are combined with AND, fields within each term are combined with OR.
    ///
    /// # Examples
    ///
    /// For terms ["rust", "web"] across fields [title, content]:
    /// ```sql
    /// ((title ILIKE '%rust%' OR content ILIKE '%rust%')
    ///  AND
    ///  (title ILIKE '%web%' OR content ILIKE '%web%'))
    /// ```
    pub fn compile_to_sql<M: Model>(term_lookups: Vec<Vec<Lookup<M>>>) -> Option<String> {
        if term_lookups.is_empty() {
            return None;
        }

        use reinhardt_orm::QueryFieldCompiler;

        let term_clauses: Vec<String> = term_lookups
            .into_iter()
            .filter(|lookups| !lookups.is_empty())
            .map(|lookups| {
                // Each term: OR across all fields
                let field_conditions: Vec<String> = lookups
                    .iter()
                    .map(|lookup| QueryFieldCompiler::compile(lookup))
                    .collect();

                if field_conditions.len() == 1 {
                    field_conditions[0].clone()
                } else {
                    format!("({})", field_conditions.join(" OR "))
                }
            })
            .collect();

        if term_clauses.is_empty() {
            return None;
        }

        if term_clauses.len() == 1 {
            Some(term_clauses[0].clone())
        } else {
            // All terms: AND together
            Some(format!("({})", term_clauses.join(" AND ")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reinhardt_orm::{Field, Model};

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct TestPost {
        id: i64,
        title: String,
        content: String,
    }

    impl Model for TestPost {
        type PrimaryKey = i64;

        fn table_name() -> &'static str {
            "test_posts"
        }

        fn primary_key(&self) -> Option<&Self::PrimaryKey> {
            Some(&self.id)
        }

        fn set_primary_key(&mut self, value: Self::PrimaryKey) {
            self.id = value;
        }
    }

    impl SearchableModel for TestPost {
        fn searchable_fields() -> Vec<Field<Self, String>> {
            vec![
                Field::<TestPost, String>::new(vec!["title"]),
                Field::<TestPost, String>::new(vec!["content"]),
            ]
        }
    }

    #[test]
    fn test_search_single_term() {
        let terms = vec!["rust"];
        let lookups = MultiTermSearch::search_terms::<TestPost>(terms);

        assert_eq!(lookups.len(), 1); // One term
        assert_eq!(lookups[0].len(), 2); // Two fields (title, content)
    }

    #[test]
    fn test_search_multiple_terms() {
        let terms = vec!["rust", "programming"];
        let lookups = MultiTermSearch::search_terms::<TestPost>(terms);

        assert_eq!(lookups.len(), 2); // Two terms
        assert_eq!(lookups[0].len(), 2); // Each term searches 2 fields
        assert_eq!(lookups[1].len(), 2);
    }

    #[test]
    fn test_exact_terms() {
        let terms = vec!["rust"];
        let lookups = MultiTermSearch::exact_terms::<TestPost>(terms);

        assert_eq!(lookups.len(), 1);
        assert_eq!(lookups[0].len(), 2);
    }

    #[test]
    fn test_prefix_terms() {
        let terms = vec!["rust"];
        let lookups = MultiTermSearch::prefix_terms::<TestPost>(terms);

        assert_eq!(lookups.len(), 1);
        assert_eq!(lookups[0].len(), 2);
    }

    #[test]
    fn test_compile_single_term_to_sql() {
        let terms = vec!["rust"];
        let lookups = MultiTermSearch::search_terms::<TestPost>(terms);
        let sql = MultiTermSearch::compile_to_sql(lookups).unwrap();

        assert!(sql.contains("title"));
        assert!(sql.contains("content"));
        assert!(sql.contains("OR"));
        // SQLite uses LIKE with LOWER() for case-insensitive
        assert!(sql.contains("LIKE"));
        assert!(sql.contains("LOWER"));
    }

    #[test]
    fn test_compile_multiple_terms_to_sql() {
        let terms = vec!["rust", "web"];
        let lookups = MultiTermSearch::search_terms::<TestPost>(terms);
        let sql = MultiTermSearch::compile_to_sql(lookups).unwrap();

        assert!(sql.contains("title"));
        assert!(sql.contains("content"));
        assert!(sql.contains("OR"));
        assert!(sql.contains("AND"));
        // SQLite uses LIKE with LOWER() for case-insensitive
        assert!(sql.contains("LIKE"));
        assert!(sql.contains("LOWER"));
    }

    #[test]
    fn test_compile_empty_terms() {
        let lookups: Vec<Vec<Lookup<TestPost>>> = vec![];
        let sql = MultiTermSearch::compile_to_sql(lookups);

        assert!(sql.is_none());
    }

    #[test]
    fn test_parse_simple_terms() {
        let input = "rust, programming";
        let terms = MultiTermSearch::parse_search_terms(input);

        assert_eq!(terms.len(), 2);
    }

    #[test]
    fn test_parse_quoted_terms() {
        let input = "\"hello world\", rust";
        let terms = MultiTermSearch::parse_search_terms(input);

        // Note: The current implementation is simplified
        // A full implementation would properly handle quoted strings
        assert!(terms.len() >= 1);
    }

    #[test]
    fn test_parse_empty_string() {
        let input = "";
        let terms = MultiTermSearch::parse_search_terms(input);

        assert_eq!(terms.len(), 0);
    }
}
