//! Integration tests for type-safe QueryFilter
//!
//! Tests the compile-time type-safe filtering system using Field<M, T> and Lookup.

mod common;

use common::{execute_and_count, execute_and_fetch, seed_test_data, setup_test_db, Post, User};
use reinhardt_filters::{
    FieldOrderingExt, FilterBackend, MultiTermSearch, QueryFilter, SearchableModel,
};
use reinhardt_orm::{Comparable, Field};
use std::collections::HashMap;

#[tokio::test]
async fn test_query_filter_simple_lookup() {
    let test_db = setup_test_db().await;
    seed_test_data(&test_db.db).await;

    // Filter posts where title contains "Rust"
    let lookup = Field::<Post, String>::new(vec!["title"]).icontains("Rust");
    let filter = QueryFilter::new().add(lookup);

    let base_sql = "SELECT * FROM posts".to_string();
    let filtered_sql = filter
        .filter_queryset(&HashMap::new(), base_sql)
        .await
        .unwrap();

    let count = execute_and_count(&test_db.db, &filtered_sql).await;
    assert_eq!(count, 3); // "Rust Tutorial", "Hello Rust", "Advanced Rust"
}

#[tokio::test]
async fn test_query_filter_multiple_and_conditions() {
    let test_db = setup_test_db().await;
    seed_test_data(&test_db.db).await;

    // Filter posts where title contains "Rust" AND author_id = 1
    let filter = QueryFilter::new()
        .add(Field::<Post, String>::new(vec!["title"]).icontains("Rust"))
        .add(Field::<Post, i64>::new(vec!["author_id"]).eq(1));

    let base_sql = "SELECT * FROM posts".to_string();
    let filtered_sql = filter
        .filter_queryset(&HashMap::new(), base_sql)
        .await
        .unwrap();

    let count = execute_and_count(&test_db.db, &filtered_sql).await;
    assert_eq!(count, 1); // Only "Hello Rust" by author 1
}

#[tokio::test]
async fn test_query_filter_with_ordering() {
    let test_db = setup_test_db().await;
    seed_test_data(&test_db.db).await;

    // Filter and order posts
    let filter = QueryFilter::new()
        .add(Field::<Post, String>::new(vec!["title"]).icontains("Rust"))
        .order_by(Field::<Post, String>::new(vec!["title"]).asc());

    let base_sql = "SELECT * FROM posts".to_string();
    let filtered_sql = filter
        .filter_queryset(&HashMap::new(), base_sql)
        .await
        .unwrap();

    let rows = execute_and_fetch(&test_db.db, &filtered_sql).await;
    assert_eq!(rows.len(), 3);

    // Check ordering: "Advanced Rust", "Hello Rust", "Rust Tutorial"
    assert_eq!(rows[0]["title"], "Advanced Rust");
    assert_eq!(rows[1]["title"], "Hello Rust");
    assert_eq!(rows[2]["title"], "Rust Tutorial");
}

#[tokio::test]
async fn test_query_filter_descending_order() {
    let test_db = setup_test_db().await;
    seed_test_data(&test_db.db).await;

    let filter = QueryFilter::new()
        .add(Field::<Post, String>::new(vec!["title"]).icontains("Rust"))
        .order_by(Field::<Post, String>::new(vec!["title"]).desc());

    let base_sql = "SELECT * FROM posts".to_string();
    let filtered_sql = filter
        .filter_queryset(&HashMap::new(), base_sql)
        .await
        .unwrap();

    let rows = execute_and_fetch(&test_db.db, &filtered_sql).await;
    assert_eq!(rows.len(), 3);

    // Check ordering: "Rust Tutorial", "Hello Rust", "Advanced Rust"
    assert_eq!(rows[0]["title"], "Rust Tutorial");
    assert_eq!(rows[1]["title"], "Hello Rust");
    assert_eq!(rows[2]["title"], "Advanced Rust");
}

#[tokio::test]
async fn test_query_filter_or_group() {
    let test_db = setup_test_db().await;
    seed_test_data(&test_db.db).await;

    // Filter posts where (title contains "Hello" OR title contains "Advanced")
    let filter = QueryFilter::new().add_or_group(vec![
        Field::<Post, String>::new(vec!["title"]).icontains("Hello"),
        Field::<Post, String>::new(vec!["title"]).icontains("Advanced"),
    ]);

    let base_sql = "SELECT * FROM posts".to_string();
    let filtered_sql = filter
        .filter_queryset(&HashMap::new(), base_sql)
        .await
        .unwrap();

    let count = execute_and_count(&test_db.db, &filtered_sql).await;
    assert_eq!(count, 3); // "Hello World", "Hello Rust", "Advanced Rust"
}

#[tokio::test]
async fn test_multi_term_search() {
    let test_db = setup_test_db().await;
    seed_test_data(&test_db.db).await;

    // Search for posts containing both "Hello" AND "Rust"
    let terms = vec!["Hello", "Rust"];
    let term_lookups = MultiTermSearch::search_terms::<Post>(terms);
    let filter = QueryFilter::new().add_multi_term(term_lookups);

    let base_sql = "SELECT * FROM posts".to_string();
    let filtered_sql = filter
        .filter_queryset(&HashMap::new(), base_sql)
        .await
        .unwrap();

    let count = execute_and_count(&test_db.db, &filtered_sql).await;
    assert_eq!(count, 1); // Only "Hello Rust"
}

#[tokio::test]
async fn test_multi_term_search_single_result() {
    let test_db = setup_test_db().await;
    seed_test_data(&test_db.db).await;

    // Search for posts containing "Rust" in any field
    let terms = vec!["Rust"];
    let term_lookups = MultiTermSearch::search_terms::<Post>(terms);
    let filter = QueryFilter::new().add_multi_term(term_lookups);

    let base_sql = "SELECT * FROM posts".to_string();
    let filtered_sql = filter
        .filter_queryset(&HashMap::new(), base_sql)
        .await
        .unwrap();

    let count = execute_and_count(&test_db.db, &filtered_sql).await;
    assert_eq!(count, 3); // All posts with "Rust"
}

#[tokio::test]
async fn test_searchable_model_trait() {
    // Test that SearchableModel trait is implemented correctly
    let fields = Post::searchable_fields();
    assert_eq!(fields.len(), 2); // title and content

    let field_names = Post::searchable_field_names();
    assert_eq!(field_names.len(), 2);
    assert!(field_names.contains(&"title".to_string()));
    assert!(field_names.contains(&"content".to_string()));
}

#[tokio::test]
async fn test_user_search() {
    let test_db = setup_test_db().await;
    seed_test_data(&test_db.db).await;

    // Search users by username
    let terms = vec!["alice"];
    let term_lookups = MultiTermSearch::search_terms::<User>(terms);
    let filter = QueryFilter::new().add_multi_term(term_lookups);

    let base_sql = "SELECT * FROM users".to_string();
    let filtered_sql = filter
        .filter_queryset(&HashMap::new(), base_sql)
        .await
        .unwrap();

    let count = execute_and_count(&test_db.db, &filtered_sql).await;
    assert_eq!(count, 1); // Only alice
}

#[tokio::test]
async fn test_exact_search() {
    let test_db = setup_test_db().await;
    seed_test_data(&test_db.db).await;

    // Exact match search
    let terms = vec!["Rust Tutorial"];
    let term_lookups = MultiTermSearch::exact_terms::<Post>(terms);
    let filter = QueryFilter::new().add_multi_term(term_lookups);

    let base_sql = "SELECT * FROM posts".to_string();
    let filtered_sql = filter
        .filter_queryset(&HashMap::new(), base_sql)
        .await
        .unwrap();

    let count = execute_and_count(&test_db.db, &filtered_sql).await;
    assert_eq!(count, 1); // Only exact match
}

#[tokio::test]
async fn test_prefix_search() {
    let test_db = setup_test_db().await;
    seed_test_data(&test_db.db).await;

    // Prefix search
    let terms = vec!["Hello"];
    let term_lookups = MultiTermSearch::prefix_terms::<Post>(terms);
    let filter = QueryFilter::new().add_multi_term(term_lookups);

    let base_sql = "SELECT * FROM posts".to_string();
    let filtered_sql = filter
        .filter_queryset(&HashMap::new(), base_sql)
        .await
        .unwrap();

    let count = execute_and_count(&test_db.db, &filtered_sql).await;
    assert_eq!(count, 2); // "Hello World", "Hello Rust"
}

#[tokio::test]
async fn test_combined_filter_and_multi_term() {
    let test_db = setup_test_db().await;
    seed_test_data(&test_db.db).await;

    // Combine regular filter with multi-term search
    let terms = vec!["Rust"];
    let term_lookups = MultiTermSearch::search_terms::<Post>(terms);
    let filter = QueryFilter::new()
        .add_multi_term(term_lookups)
        .add(Field::<Post, i64>::new(vec!["author_id"]).eq(1))
        .order_by(Field::<Post, String>::new(vec!["title"]).asc());

    let base_sql = "SELECT * FROM posts".to_string();
    let filtered_sql = filter
        .filter_queryset(&HashMap::new(), base_sql)
        .await
        .unwrap();

    let rows = execute_and_fetch(&test_db.db, &filtered_sql).await;
    assert_eq!(rows.len(), 1); // Only "Hello Rust" by author 1
    assert_eq!(rows[0]["title"], "Hello Rust");
}

#[tokio::test]
async fn test_multiple_ordering() {
    let test_db = setup_test_db().await;
    seed_test_data(&test_db.db).await;

    // Order by author_id DESC, then title ASC
    let filter = QueryFilter::new()
        .order_by(Field::<Post, i64>::new(vec!["author_id"]).desc())
        .order_by(Field::<Post, String>::new(vec!["title"]).asc());

    let base_sql = "SELECT * FROM posts".to_string();
    let filtered_sql = filter
        .filter_queryset(&HashMap::new(), base_sql)
        .await
        .unwrap();

    let rows = execute_and_fetch(&test_db.db, &filtered_sql).await;
    assert_eq!(rows.len(), 4);

    // First should be author_id=3, then author_id=2, then author_id=1 (two posts)
    assert_eq!(rows[0]["author_id"], "3"); // Charlie
    assert_eq!(rows[1]["author_id"], "2"); // Bob
}

#[tokio::test]
async fn test_comparison_operators() {
    let test_db = setup_test_db().await;
    seed_test_data(&test_db.db).await;

    // Test gte (greater than or equal)
    let filter = QueryFilter::new().add(Field::<Post, i64>::new(vec!["id"]).gte(3));

    let base_sql = "SELECT * FROM posts".to_string();
    let filtered_sql = filter
        .filter_queryset(&HashMap::new(), base_sql)
        .await
        .unwrap();

    let count = execute_and_count(&test_db.db, &filtered_sql).await;
    assert_eq!(count, 2); // Posts with id >= 3
}

#[tokio::test]
async fn test_string_operations() {
    let test_db = setup_test_db().await;
    seed_test_data(&test_db.db).await;

    // Test startswith
    let filter =
        QueryFilter::new().add(Field::<Post, String>::new(vec!["title"]).startswith("Hello"));

    let base_sql = "SELECT * FROM posts".to_string();
    let filtered_sql = filter
        .filter_queryset(&HashMap::new(), base_sql)
        .await
        .unwrap();

    let count = execute_and_count(&test_db.db, &filtered_sql).await;
    assert_eq!(count, 2); // "Hello World", "Hello Rust"
}
