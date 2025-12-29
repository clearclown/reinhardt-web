//! URL configuration for examples-tutorial-basis project
//!
//! The `routes` function defines all URL patterns for this project.

use reinhardt::prelude::*;
use reinhardt::routes;

#[routes]
pub fn routes() -> UnifiedRouter {
	UnifiedRouter::new().mount("/polls/", crate::apps::polls::urls::routes())
}
