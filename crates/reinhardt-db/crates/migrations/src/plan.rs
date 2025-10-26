//! Migration execution plan

use crate::{Migration, Result};

/// Migration execution plan
#[derive(Debug, Clone)]
pub struct MigrationPlan {
    pub migrations: Vec<Migration>,
}

impl MigrationPlan {
    /// Create a new empty migration plan
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_migrations::MigrationPlan;
    ///
    /// let plan = MigrationPlan::new();
    /// assert_eq!(plan.migrations.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }
    /// Add a migration to this plan
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_migrations::{Migration, MigrationPlan};
    ///
    /// let migration = Migration::new("0001_initial", "myapp");
    /// let plan = MigrationPlan::new().add(migration);
    ///
    /// assert_eq!(plan.migrations.len(), 1);
    /// ```
    pub fn add(mut self, migration: Migration) -> Self {
        self.migrations.push(migration);
        self
    }
    /// Sort migrations by dependencies (topological sort)
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_migrations::{Migration, MigrationPlan};
    ///
    /// let migration1 = Migration::new("0001_initial", "myapp");
    /// let migration2 = Migration::new("0002_add_field", "myapp")
    ///     .add_dependency("myapp", "0001_initial");
    ///
    /// let mut plan = MigrationPlan::new()
    ///     .add(migration2.clone())
    ///     .add(migration1.clone());
    ///
    /// plan.sort().unwrap();
    ///
    // After sorting, 0001 should come before 0002
    /// assert_eq!(plan.migrations[0].name, "0001_initial");
    /// assert_eq!(plan.migrations[1].name, "0002_add_field");
    /// ```
    pub fn sort(&mut self) -> Result<()> {
        // Simple topological sort
        let mut sorted = Vec::new();
        let mut remaining: Vec<_> = self.migrations.drain(..).collect();

        while !remaining.is_empty() {
            let mut found_any = false;

            let mut i = 0;
            while i < remaining.len() {
                let migration = &remaining[i];
                let all_deps_met = migration.dependencies.iter().all(|(app, name)| {
                    sorted
                        .iter()
                        .any(|m: &Migration| m.app_label == *app && m.name == *name)
                });

                if all_deps_met {
                    sorted.push(remaining.remove(i));
                    found_any = true;
                } else {
                    i += 1;
                }
            }

            if !found_any && !remaining.is_empty() {
                return Err(crate::MigrationError::DependencyError(
                    "Circular dependency detected".to_string(),
                ));
            }
        }

        self.migrations = sorted;
        Ok(())
    }
}

impl Default for MigrationPlan {
    fn default() -> Self {
        Self::new()
    }
}
