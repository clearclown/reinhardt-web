use reinhardt_auth::{DefaultUser, PermissionsMixin};
use uuid::Uuid;

#[test]
fn test_permissions_mixin_has_perm() {
	let user = DefaultUser {
		id: Uuid::new_v4(),
		username: "alice".to_string(),
		email: "alice@example.com".to_string(),
		first_name: "Alice".to_string(),
		last_name: "Smith".to_string(),
		password_hash: None,
		last_login: None,
		is_active: true,
		is_staff: true,
		is_superuser: false,
		date_joined: chrono::Utc::now(),
		user_permissions: vec!["blog.add_post".to_string(), "blog.change_post".to_string()],
		groups: Vec::new(),
	};

	assert!(user.has_perm("blog.add_post"));
	assert!(user.has_perm("blog.change_post"));
	assert!(!user.has_perm("blog.delete_post"));
}

#[test]
fn test_permissions_mixin_superuser_bypass() {
	let superuser = DefaultUser {
		id: Uuid::new_v4(),
		username: "admin".to_string(),
		email: "admin@example.com".to_string(),
		first_name: "Admin".to_string(),
		last_name: "User".to_string(),
		password_hash: None,
		last_login: None,
		is_active: true,
		is_staff: true,
		is_superuser: true,
		date_joined: chrono::Utc::now(),
		user_permissions: Vec::new(),
		groups: Vec::new(),
	};

	// Superuser has all permissions, even if not explicitly granted
	assert!(superuser.has_perm("blog.add_post"));
	assert!(superuser.has_perm("blog.delete_post"));
	assert!(superuser.has_perm("any.permission"));
}

#[test]
fn test_permissions_mixin_has_module_perms() {
	let user = DefaultUser {
		id: Uuid::new_v4(),
		username: "bob".to_string(),
		email: "bob@example.com".to_string(),
		first_name: "Bob".to_string(),
		last_name: "Johnson".to_string(),
		password_hash: None,
		last_login: None,
		is_active: true,
		is_staff: false,
		is_superuser: false,
		date_joined: chrono::Utc::now(),
		user_permissions: vec!["blog.add_post".to_string(), "blog.change_post".to_string()],
		groups: Vec::new(),
	};

	assert!(user.has_module_perms("blog"));
	assert!(!user.has_module_perms("shop"));
}

#[test]
fn test_permissions_mixin_group_permissions() {
	let user = DefaultUser {
		id: Uuid::new_v4(),
		username: "charlie".to_string(),
		email: "charlie@example.com".to_string(),
		first_name: "Charlie".to_string(),
		last_name: "Brown".to_string(),
		password_hash: None,
		last_login: None,
		is_active: true,
		is_staff: false,
		is_superuser: false,
		date_joined: chrono::Utc::now(),
		user_permissions: Vec::new(),
		groups: vec!["editors".to_string(), "moderators".to_string()],
	};

	assert_eq!(user.groups().len(), 2);
	assert!(user.groups().contains(&"editors".to_string()));
	assert!(user.groups().contains(&"moderators".to_string()));
}

#[test]
fn test_permissions_mixin_get_all_permissions() {
	let user = DefaultUser {
		id: Uuid::new_v4(),
		username: "dave".to_string(),
		email: "dave@example.com".to_string(),
		first_name: "Dave".to_string(),
		last_name: "Wilson".to_string(),
		password_hash: None,
		last_login: None,
		is_active: true,
		is_staff: false,
		is_superuser: false,
		date_joined: chrono::Utc::now(),
		user_permissions: vec![
			"blog.add_post".to_string(),
			"blog.change_post".to_string(),
			"blog.delete_post".to_string(),
		],
		groups: Vec::new(),
	};

	let all_perms = user.get_all_permissions();
	assert_eq!(all_perms.len(), 3);
	assert!(all_perms.contains("blog.add_post"));
	assert!(all_perms.contains("blog.change_post"));
	assert!(all_perms.contains("blog.delete_post"));
}

#[test]
fn test_permissions_mixin_no_permissions() {
	let user = DefaultUser {
		id: Uuid::new_v4(),
		username: "eve".to_string(),
		email: "eve@example.com".to_string(),
		first_name: "Eve".to_string(),
		last_name: "Anderson".to_string(),
		password_hash: None,
		last_login: None,
		is_active: true,
		is_staff: false,
		is_superuser: false,
		date_joined: chrono::Utc::now(),
		user_permissions: Vec::new(),
		groups: Vec::new(),
	};

	assert!(!user.has_perm("blog.add_post"));
	assert!(!user.has_module_perms("blog"));
	assert_eq!(user.get_all_permissions().len(), 0);
}
