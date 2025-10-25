//! {{ app_name }} application module
//!
//! A Model-Template-View application

use reinhardt::prelude::*;

pub struct {{ camel_case_app_name }}Config;

impl AppConfig for {{ camel_case_app_name }}Config {
    fn name(&self) -> &str {
        "{{ app_name }}"
    }

    fn label(&self) -> &str {
        "{{ app_name }}"
    }
}
