//! # Many-to-Many Relationship Support
//!
//! SQLAlchemy-inspired many-to-many relationship implementation.
//!
//! This module is inspired by SQLAlchemy's relationship patterns
//! Copyright 2005-2025 SQLAlchemy authors and contributors
//! Licensed under MIT License. See THIRD-PARTY-NOTICES for details.

use crate::Model;
use std::marker::PhantomData;

/// Association table definition for many-to-many relationships
#[derive(Debug, Clone)]
pub struct AssociationTable {
    /// Table name
    pub table_name: String,

    /// Left side foreign key column
    pub left_column: String,

    /// Right side foreign key column
    pub right_column: String,

    /// Additional columns in the association table
    pub extra_columns: Vec<(String, String)>, // (column_name, column_type)
}

impl AssociationTable {
    /// Create a new association table definition
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_orm::many_to_many::AssociationTable;
    ///
    /// let table = AssociationTable::new("student_courses", "student_id", "course_id");
    /// let sql = table.to_create_sql();
    ///
    /// assert!(sql.contains("CREATE TABLE student_courses"));
    /// assert!(sql.contains("student_id INTEGER NOT NULL"));
    /// assert!(sql.contains("course_id INTEGER NOT NULL"));
    /// assert!(sql.contains("PRIMARY KEY (student_id, course_id)"));
    /// ```
    pub fn new(
        table_name: impl Into<String>,
        left_column: impl Into<String>,
        right_column: impl Into<String>,
    ) -> Self {
        Self {
            table_name: table_name.into(),
            left_column: left_column.into(),
            right_column: right_column.into(),
            extra_columns: Vec::new(),
        }
    }
    /// Add extra column to the association table
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_orm::many_to_many::AssociationTable;
    ///
    /// let table = AssociationTable::new("student_courses", "student_id", "course_id")
    ///     .with_column("enrolled_at", "TIMESTAMP")
    ///     .with_column("grade", "VARCHAR(2)");
    ///
    /// let sql = table.to_create_sql();
    /// assert!(sql.contains("enrolled_at TIMESTAMP"));
    /// assert!(sql.contains("grade VARCHAR(2)"));
    /// ```
    pub fn with_column(mut self, name: impl Into<String>, type_: impl Into<String>) -> Self {
        self.extra_columns.push((name.into(), type_.into()));
        self
    }
    /// Generate CREATE TABLE SQL for the association table
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_orm::many_to_many::AssociationTable;
    ///
    /// let table = AssociationTable::new("user_roles", "user_id", "role_id");
    /// let sql = table.to_create_sql();
    ///
    /// assert_eq!(sql, "CREATE TABLE user_roles (\n  user_id INTEGER NOT NULL,\n  role_id INTEGER NOT NULL,\n  PRIMARY KEY (user_id, role_id)\n)");
    /// ```
    pub fn to_create_sql(&self) -> String {
        let mut columns = vec![
            format!("{} INTEGER NOT NULL", self.left_column),
            format!("{} INTEGER NOT NULL", self.right_column),
        ];

        for (name, type_) in &self.extra_columns {
            columns.push(format!("{} {}", name, type_));
        }

        columns.push(format!(
            "PRIMARY KEY ({}, {})",
            self.left_column, self.right_column
        ));

        format!(
            "CREATE TABLE {} (\n  {}\n)",
            self.table_name,
            columns.join(",\n  ")
        )
    }
}

/// Many-to-many relationship between two models
pub struct ManyToMany<L: Model, R: Model> {
    /// Association table definition
    association_table: AssociationTable,

    /// Loading strategy (lazy, eager, etc.)
    lazy: bool,

    /// Back reference name
    back_populates: Option<String>,

    /// Cascade options
    cascade: Vec<String>,

    _phantom_left: PhantomData<L>,
    _phantom_right: PhantomData<R>,
}

impl<L: Model, R: Model> ManyToMany<L, R> {
    /// Create a new many-to-many relationship
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_orm::many_to_many::{AssociationTable, ManyToMany};
    /// use reinhardt_orm::Model;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// struct Student { id: Option<i64>, name: String }
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// struct Course { id: Option<i64>, title: String }
    ///
    /// impl Model for Student {
    ///     type PrimaryKey = i64;
    ///     fn table_name() -> &'static str { "students" }
    ///     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
    ///     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
    /// }
    ///
    /// impl Model for Course {
    ///     type PrimaryKey = i64;
    ///     fn table_name() -> &'static str { "courses" }
    ///     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
    ///     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
    /// }
    ///
    /// let assoc = AssociationTable::new("student_courses", "student_id", "course_id");
    /// let m2m = ManyToMany::<Student, Course>::new(assoc);
    /// let join_sql = m2m.join_sql();
    ///
    /// assert!(join_sql.contains("JOIN student_courses"));
    /// assert!(join_sql.contains("students"));
    /// assert!(join_sql.contains("courses"));
    /// ```
    pub fn new(association_table: AssociationTable) -> Self {
        Self {
            association_table,
            lazy: true,
            back_populates: None,
            cascade: Vec::new(),
            _phantom_left: PhantomData,
            _phantom_right: PhantomData,
        }
    }
    /// Set eager loading
    ///
    pub fn eager(mut self) -> Self {
        self.lazy = false;
        self
    }
    /// Set back reference
    ///
    pub fn back_populates(mut self, name: impl Into<String>) -> Self {
        self.back_populates = Some(name.into());
        self
    }
    /// Add cascade option
    ///
    pub fn cascade(mut self, option: impl Into<String>) -> Self {
        self.cascade.push(option.into());
        self
    }
    /// Generate SQL for joining through the association table
    ///
    pub fn join_sql(&self) -> String {
        format!(
            "JOIN {} ON {}.{} = {}.{} JOIN {} ON {}.{} = {}.{}",
            self.association_table.table_name,
            L::table_name(),
            "id", // Assuming primary key is 'id'
            self.association_table.table_name,
            self.association_table.left_column,
            R::table_name(),
            self.association_table.table_name,
            self.association_table.right_column,
            R::table_name(),
            "id" // Assuming primary key is 'id'
        )
    }
    /// Generate SQL for adding a relationship
    ///
    pub fn add_sql(&self, left_id: i64, right_id: i64) -> String {
        format!(
            "INSERT INTO {} ({}, {}) VALUES ({}, {})",
            self.association_table.table_name,
            self.association_table.left_column,
            self.association_table.right_column,
            left_id,
            right_id
        )
    }
    /// Generate SQL for removing a relationship
    ///
    pub fn remove_sql(&self, left_id: i64, right_id: i64) -> String {
        format!(
            "DELETE FROM {} WHERE {} = {} AND {} = {}",
            self.association_table.table_name,
            self.association_table.left_column,
            left_id,
            self.association_table.right_column,
            right_id
        )
    }
    /// Get association table reference
    ///
    pub fn table(&self) -> &AssociationTable {
        &self.association_table
    }
}
/// Helper function to create an association table
///
pub fn association_table(
    table_name: impl Into<String>,
    left_column: impl Into<String>,
    right_column: impl Into<String>,
) -> AssociationTable {
    AssociationTable::new(table_name, left_column, right_column)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reinhardt_validators::TableName;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Student {
        id: Option<i64>,
        name: String,
    }

    const STUDENT_TABLE: TableName = TableName::new_const("students");

    impl Model for Student {
        type PrimaryKey = i64;

        fn table_name() -> &'static str {
            STUDENT_TABLE.as_str()
        }

        fn primary_key(&self) -> Option<&Self::PrimaryKey> {
            self.id.as_ref()
        }

        fn set_primary_key(&mut self, value: Self::PrimaryKey) {
            self.id = Some(value);
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Course {
        id: Option<i64>,
        title: String,
    }

    const COURSE_TABLE: TableName = TableName::new_const("courses");

    impl Model for Course {
        type PrimaryKey = i64;

        fn table_name() -> &'static str {
            COURSE_TABLE.as_str()
        }

        fn primary_key(&self) -> Option<&Self::PrimaryKey> {
            self.id.as_ref()
        }

        fn set_primary_key(&mut self, value: Self::PrimaryKey) {
            self.id = Some(value);
        }
    }

    #[test]
    fn test_association_table() {
        let table = AssociationTable::new("student_courses", "student_id", "course_id");
        let sql = table.to_create_sql();

        assert!(sql.contains("CREATE TABLE student_courses"));
        assert!(sql.contains("student_id INTEGER NOT NULL"));
        assert!(sql.contains("course_id INTEGER NOT NULL"));
        assert!(sql.contains("PRIMARY KEY (student_id, course_id)"));
    }

    #[test]
    fn test_association_table_with_extra_columns() {
        let table = AssociationTable::new("student_courses", "student_id", "course_id")
            .with_column("enrolled_at", "TIMESTAMP")
            .with_column("grade", "VARCHAR(2)");

        let sql = table.to_create_sql();
        assert!(sql.contains("enrolled_at TIMESTAMP"));
        assert!(sql.contains("grade VARCHAR(2)"));
    }

    #[test]
    fn test_many_to_many_join() {
        let assoc = AssociationTable::new("student_courses", "student_id", "course_id");
        let m2m = ManyToMany::<Student, Course>::new(assoc);

        let join_sql = m2m.join_sql();
        assert!(join_sql.contains("JOIN student_courses"));
        assert!(join_sql.contains("students"));
        assert!(join_sql.contains("courses"));
    }

    #[test]
    fn test_many_to_many_add() {
        let assoc = AssociationTable::new("student_courses", "student_id", "course_id");
        let m2m = ManyToMany::<Student, Course>::new(assoc);

        let sql = m2m.add_sql(1, 10);
        assert_eq!(
            sql,
            "INSERT INTO student_courses (student_id, course_id) VALUES (1, 10)"
        );
    }

    #[test]
    fn test_many_to_many_remove() {
        let assoc = AssociationTable::new("student_courses", "student_id", "course_id");
        let m2m = ManyToMany::<Student, Course>::new(assoc);

        let sql = m2m.remove_sql(1, 10);
        assert_eq!(
            sql,
            "DELETE FROM student_courses WHERE student_id = 1 AND course_id = 10"
        );
    }
}
