use reinhardt_auth::{DefaultUser, FullUser};
use uuid::Uuid;

#[test]
fn test_full_user_get_full_name() {
    let user = DefaultUser {
        id: Uuid::new_v4(),
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Smith".to_string(),
        password_hash: None,
        last_login: None,
        is_active: true,
        is_staff: false,
        is_superuser: false,
        date_joined: chrono::Utc::now(),
        user_permissions: Vec::new(),
        groups: Vec::new(),
    };

    assert_eq!(user.get_full_name(), "Alice Smith");
}

#[test]
fn test_full_user_get_short_name() {
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
        user_permissions: Vec::new(),
        groups: Vec::new(),
    };

    assert_eq!(user.get_short_name(), "Bob");
}

#[test]
fn test_full_user_empty_names() {
    let user = DefaultUser {
        id: Uuid::new_v4(),
        username: "user123".to_string(),
        email: "user@example.com".to_string(),
        first_name: String::new(),
        last_name: String::new(),
        password_hash: None,
        last_login: None,
        is_active: true,
        is_staff: false,
        is_superuser: false,
        date_joined: chrono::Utc::now(),
        user_permissions: Vec::new(),
        groups: Vec::new(),
    };

    // get_full_name should return empty string when both names are empty
    assert_eq!(user.get_full_name(), "");
    assert_eq!(user.get_short_name(), "");
}

#[test]
fn test_full_user_staff_and_superuser_flags() {
    // Regular user
    let regular_user = DefaultUser {
        id: Uuid::new_v4(),
        username: "regular".to_string(),
        email: "regular@example.com".to_string(),
        first_name: "Regular".to_string(),
        last_name: "User".to_string(),
        password_hash: None,
        last_login: None,
        is_active: true,
        is_staff: false,
        is_superuser: false,
        date_joined: chrono::Utc::now(),
        user_permissions: Vec::new(),
        groups: Vec::new(),
    };

    assert!(!regular_user.is_staff());
    assert!(!regular_user.is_superuser());

    // Staff user (not superuser)
    let staff_user = DefaultUser {
        id: Uuid::new_v4(),
        username: "staff".to_string(),
        email: "staff@example.com".to_string(),
        first_name: "Staff".to_string(),
        last_name: "User".to_string(),
        password_hash: None,
        last_login: None,
        is_active: true,
        is_staff: true,
        is_superuser: false,
        date_joined: chrono::Utc::now(),
        user_permissions: Vec::new(),
        groups: Vec::new(),
    };

    assert!(staff_user.is_staff());
    assert!(!staff_user.is_superuser());

    // Superuser (also staff)
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

    assert!(superuser.is_staff());
    assert!(superuser.is_superuser());
}
