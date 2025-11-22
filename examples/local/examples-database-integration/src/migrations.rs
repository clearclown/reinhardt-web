use reinhardt::Migration;

// Import migration modules
mod migrations {
	pub mod initial {
		include!("../migrations/0001_initial.rs");
	}
	pub mod create_todos {
		include!("../migrations/0002_create_todos.rs");
	}
}

/// Returns all migrations in order
///
/// Migrations are executed in the order they appear in this vector.
pub fn all_migrations() -> Vec<Migration> {
	vec![
		migrations::initial::migration(),
		migrations::create_todos::migration(),
	]
}
