//! Migration provider for examples-twitter application.
//!
//! This module provides all migrations for the Twitter example application,
//! enabling TestContainers-based tests to apply migrations automatically.

use reinhardt::db::migrations::{Migration, MigrationProvider};

// Import migration modules
// mod auth_migrations {
// 	include!("../migrations/auth/migrations/0001_initial.rs");
// }
//
// mod profile_migrations {
// 	include!("../migrations/profile/migrations/0001_initial.rs");
// }
//
// mod dm_migrations {
// 	include!("../migrations/dm/migrations/0001_initial.rs");
// }

/// Migration provider for the Twitter example application.
///
/// Provides migrations for all apps in dependency order:
/// 1. auth - User model (base)
/// 2. profile - Profile model (depends on auth)
/// 3. dm - DM Room and Message models (depends on auth)
pub struct TwitterMigrations;

impl MigrationProvider for TwitterMigrations {
	fn migrations() -> Vec<Migration> {
		vec![
			// Auth migrations first (no dependencies)
			// auth_migrations::migration(),
			// Profile migrations (depends on auth_user)
			// profile_migrations::migration(),
			// DM migrations (depends on auth_user)
			// dm_migrations::migration(),
		]
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;
//
// 	#[test]
// 	fn test_migrations_count() {
// 		let migrations = TwitterMigrations::migrations();
// 		assert_eq!(migrations.len(), 3);
// 	}
//
// 	#[test]
// 	fn test_migrations_order() {
// 		let migrations = TwitterMigrations::migrations();
// 		assert_eq!(migrations[0].app_label, "auth");
// 		assert_eq!(migrations[1].app_label, "profile");
// 		assert_eq!(migrations[2].app_label, "dm");
// 	}
// }
