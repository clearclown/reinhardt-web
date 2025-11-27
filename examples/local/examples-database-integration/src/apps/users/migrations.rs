//! Migrations for users app

pub mod _0001_initial;

// Register migrations in global registry via linkme
reinhardt::collect_migrations!(
	app_label = "users",
	_0001_initial,
);
