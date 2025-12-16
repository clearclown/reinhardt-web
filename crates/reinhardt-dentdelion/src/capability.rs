//! Plugin capability system.
//!
//! Capabilities define what functionality a plugin can provide or consume.
//! Plugins must declare their capabilities, and only declared capabilities
//! are activated at runtime.
//!
//! # Design
//!
//! The capability system uses a two-tier approach:
//! - [`PluginCapability`]: Core framework capabilities (compile-time optimized)
//! - [`Capability`]: Wrapper supporting both core and custom capabilities
//!
//! # Example
//!
//! ```ignore
//! use reinhardt_dentdelion::capability::{Capability, PluginCapability};
//!
//! // Core capabilities
//! let middleware = Capability::Core(PluginCapability::Middleware);
//! let models = Capability::Core(PluginCapability::Models);
//!
//! // Custom capability
//! let custom = Capability::Custom("my-custom-feature".to_string());
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Core plugin capabilities defined by the framework.
///
/// These are the standard capabilities that plugins can provide.
/// Using an enum for core capabilities provides:
/// - Compile-time type safety
/// - Efficient storage and comparison
/// - Pattern matching support
///
/// The `#[non_exhaustive]` attribute allows adding new capabilities
/// in future versions without breaking existing code.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PluginCapability {
	/// Provides HTTP middleware components.
	///
	/// Plugins with this capability can intercept and modify
	/// HTTP requests and responses.
	Middleware,

	/// Provides database models and migrations.
	///
	/// Plugins with this capability can define database tables
	/// and manage their schema through migrations.
	///
	/// **Note**: Only available for static plugins (Rust crates).
	/// WASM plugins cannot provide models due to compile-time requirements.
	Models,

	/// Provides CLI management commands.
	///
	/// Plugins with this capability can add commands to
	/// `reinhardt-admin-cli`.
	Commands,

	/// Provides REST API ViewSets.
	///
	/// Plugins with this capability can register ViewSets
	/// that handle REST API endpoints.
	ViewSets,

	/// Provides custom signals.
	///
	/// Plugins with this capability can define and emit
	/// custom signals, or subscribe to existing signals.
	Signals,

	/// Provides DI services.
	///
	/// Plugins with this capability can register services
	/// in the dependency injection container.
	Services,

	/// Provides authentication backends.
	///
	/// Plugins with this capability can implement authentication
	/// mechanisms (JWT, OAuth, etc.).
	Auth,

	/// Provides template engines or filters.
	///
	/// Plugins with this capability can extend the template
	/// rendering system.
	Templates,

	/// Provides static file handling.
	///
	/// Plugins with this capability can serve or process
	/// static files.
	StaticFiles,

	/// Provides URL routing.
	///
	/// Plugins with this capability can register custom
	/// routes and route handlers.
	Routing,

	/// Provides signal receivers.
	///
	/// Plugins with this capability can listen to and
	/// respond to signals from other parts of the system.
	SignalReceivers,

	/// Provides HTTP handlers/views.
	///
	/// Plugins with this capability can handle HTTP requests
	/// directly (not through ViewSets).
	Handlers,

	/// Provides network/HTTP access.
	///
	/// Plugins with this capability can make external HTTP requests
	/// via the host API.
	NetworkAccess,

	/// Provides database access.
	///
	/// Plugins with this capability can execute SQL queries and
	/// statements via the host API.
	DatabaseAccess,
}

impl PluginCapability {
	/// Returns all core capabilities.
	pub fn all() -> &'static [Self] {
		&[
			Self::Middleware,
			Self::Models,
			Self::Commands,
			Self::ViewSets,
			Self::Signals,
			Self::Services,
			Self::Auth,
			Self::Templates,
			Self::StaticFiles,
			Self::Routing,
			Self::SignalReceivers,
			Self::Handlers,
			Self::NetworkAccess,
			Self::DatabaseAccess,
		]
	}

	/// Returns the string identifier for this capability.
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Middleware => "middleware",
			Self::Models => "models",
			Self::Commands => "commands",
			Self::ViewSets => "viewsets",
			Self::Signals => "signals",
			Self::Services => "services",
			Self::Auth => "auth",
			Self::Templates => "templates",
			Self::StaticFiles => "static_files",
			Self::Routing => "routing",
			Self::SignalReceivers => "signal_receivers",
			Self::Handlers => "handlers",
			Self::NetworkAccess => "network_access",
			Self::DatabaseAccess => "database_access",
		}
	}

	/// Returns whether this capability is available for WASM plugins.
	///
	/// Some capabilities require compile-time integration and are
	/// therefore not available for dynamic (WASM) plugins.
	pub fn is_wasm_compatible(&self) -> bool {
		match self {
			// Models require compile-time linkme registration
			Self::Models => false,
			// All other capabilities can be provided by WASM plugins
			_ => true,
		}
	}
}

impl fmt::Display for PluginCapability {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

impl std::str::FromStr for PluginCapability {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"middleware" => Ok(Self::Middleware),
			"models" => Ok(Self::Models),
			"commands" => Ok(Self::Commands),
			"viewsets" => Ok(Self::ViewSets),
			"signals" => Ok(Self::Signals),
			"services" => Ok(Self::Services),
			"auth" => Ok(Self::Auth),
			"templates" => Ok(Self::Templates),
			"static_files" | "staticfiles" => Ok(Self::StaticFiles),
			"routing" => Ok(Self::Routing),
			"signal_receivers" | "signalreceivers" => Ok(Self::SignalReceivers),
			"handlers" => Ok(Self::Handlers),
			"network_access" | "networkaccess" => Ok(Self::NetworkAccess),
			"database_access" | "databaseaccess" => Ok(Self::DatabaseAccess),
			_ => Err(format!("unknown capability: {s}")),
		}
	}
}

/// Extended capability wrapper supporting custom capabilities.
///
/// This allows third-party plugins to define custom capabilities
/// while maintaining efficiency for core capabilities.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Capability {
	/// Core framework capability.
	Core(PluginCapability),
	/// Custom capability defined by third-party plugins.
	Custom(String),
}

impl Capability {
	/// Creates a new core capability.
	pub fn core(capability: PluginCapability) -> Self {
		Self::Core(capability)
	}

	/// Creates a new custom capability.
	pub fn custom(name: impl Into<String>) -> Self {
		Self::Custom(name.into())
	}

	/// Returns the string identifier for this capability.
	pub fn as_str(&self) -> &str {
		match self {
			Self::Core(cap) => cap.as_str(),
			Self::Custom(name) => name.as_str(),
		}
	}

	/// Returns whether this is a core capability.
	pub fn is_core(&self) -> bool {
		matches!(self, Self::Core(_))
	}

	/// Returns whether this capability is available for WASM plugins.
	pub fn is_wasm_compatible(&self) -> bool {
		match self {
			Self::Core(cap) => cap.is_wasm_compatible(),
			// Custom capabilities are assumed to be WASM-compatible
			// unless explicitly stated otherwise
			Self::Custom(_) => true,
		}
	}
}

impl fmt::Display for Capability {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

impl From<PluginCapability> for Capability {
	fn from(cap: PluginCapability) -> Self {
		Self::Core(cap)
	}
}

impl std::str::FromStr for Capability {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// Try to parse as core capability first
		if let Ok(core) = s.parse::<PluginCapability>() {
			Ok(Self::Core(core))
		} else {
			// Treat as custom capability
			Ok(Self::Custom(s.to_string()))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_plugin_capability_display() {
		assert_eq!(PluginCapability::Middleware.to_string(), "middleware");
		assert_eq!(PluginCapability::Models.to_string(), "models");
		assert_eq!(PluginCapability::StaticFiles.to_string(), "static_files");
	}

	#[test]
	fn test_plugin_capability_from_str() {
		assert_eq!(
			"middleware".parse::<PluginCapability>().unwrap(),
			PluginCapability::Middleware
		);
		assert_eq!(
			"MODELS".parse::<PluginCapability>().unwrap(),
			PluginCapability::Models
		);
		assert!("unknown".parse::<PluginCapability>().is_err());
	}

	#[test]
	fn test_capability_from_str() {
		assert_eq!(
			"middleware".parse::<Capability>().unwrap(),
			Capability::Core(PluginCapability::Middleware)
		);
		assert_eq!(
			"custom-feature".parse::<Capability>().unwrap(),
			Capability::Custom("custom-feature".to_string())
		);
	}

	#[test]
	fn test_wasm_compatibility() {
		assert!(PluginCapability::Middleware.is_wasm_compatible());
		assert!(PluginCapability::Commands.is_wasm_compatible());
		assert!(!PluginCapability::Models.is_wasm_compatible());
	}
}
