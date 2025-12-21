//! Global state management
//!
//! This module provides reactive global state for the application.

use crate::shared::types::UserInfo;
use reinhardt_pages::Signal;
use std::cell::RefCell;

// Global authentication state
thread_local! {
	static AUTH_STATE: RefCell<Option<Signal<Option<UserInfo>>>> = const { RefCell::new(None) };
}

/// Initialize the authentication state
///
/// This must be called once at application startup.
pub fn init_auth_state() {
	AUTH_STATE.with(|state| {
		*state.borrow_mut() = Some(Signal::new(None));
	});
}

/// Get the authentication state signal
///
/// # Panics
///
/// Panics if `init_auth_state()` has not been called first.
pub fn auth_state() -> Signal<Option<UserInfo>> {
	AUTH_STATE.with(|state| {
		state
			.borrow()
			.as_ref()
			.expect("Auth state not initialized. Call init_auth_state() first.")
			.clone()
	})
}

/// Set the current authenticated user
pub fn set_current_user(user: Option<UserInfo>) {
	auth_state().set(user);
}

/// Get the current authenticated user
pub fn get_current_user() -> Option<UserInfo> {
	auth_state().get()
}

/// Check if a user is currently authenticated
pub fn is_authenticated() -> bool {
	auth_state().get().is_some()
}
