//! Base command trait
//!
//! Defines the interface for all management commands.

use crate::{CommandContext, CommandResult};
use async_trait::async_trait;

/// Base command trait
///
/// All management commands must implement this trait.
#[async_trait]
pub trait BaseCommand: Send + Sync {
    /// Get the command name
    fn name(&self) -> &str;

    /// Get the command description
    fn description(&self) -> &str {
        "No description available"
    }

    /// Get the command help text
    fn help(&self) -> &str {
        self.description()
    }

    /// Define command arguments
    fn arguments(&self) -> Vec<CommandArgument> {
        Vec::new()
    }

    /// Define command options/flags
    fn options(&self) -> Vec<CommandOption> {
        Vec::new()
    }

    /// Execute the command
    async fn execute(&self, ctx: &CommandContext) -> CommandResult<()>;

    /// Called before execute
    async fn before_execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
        Ok(())
    }

    /// Called after execute
    async fn after_execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
        Ok(())
    }

    /// Run the full command lifecycle
    async fn run(&self, ctx: &CommandContext) -> CommandResult<()> {
        self.before_execute(ctx).await?;
        self.execute(ctx).await?;
        self.after_execute(ctx).await?;
        Ok(())
    }
}

/// Command argument definition
#[derive(Debug, Clone)]
pub struct CommandArgument {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub default: Option<String>,
}

impl CommandArgument {
    /// Create a new required argument
    ///
    pub fn required(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            required: true,
            default: None,
        }
    }
    /// Create a new optional argument
    ///
    pub fn optional(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            required: false,
            default: None,
        }
    }
    /// Set a default value
    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }
}

/// Command option/flag definition
#[derive(Debug, Clone)]
pub struct CommandOption {
    pub short: Option<char>,
    pub long: String,
    pub description: String,
    pub takes_value: bool,
    pub required: bool,
    pub default: Option<String>,
    pub multiple: bool,
}

impl CommandOption {
    /// Create a new flag (boolean option)
    ///
    pub fn flag(
        short: Option<char>,
        long: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            short,
            long: long.into(),
            description: description.into(),
            takes_value: false,
            required: false,
            default: None,
            multiple: false,
        }
    }
    /// Create a new option that takes a value
    ///
    pub fn option(
        short: Option<char>,
        long: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            short,
            long: long.into(),
            description: description.into(),
            takes_value: true,
            required: false,
            default: None,
            multiple: false,
        }
    }
    /// Make this option required
    ///
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    /// Set a default value
    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }
    /// Allow this option to accept multiple values
    ///
    pub fn multi(mut self) -> Self {
        self.multiple = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCommand;

    #[async_trait]
    impl BaseCommand for TestCommand {
        fn name(&self) -> &str {
            "test"
        }

        fn description(&self) -> &str {
            "A test command"
        }

        async fn execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_command_basic() {
        let cmd = TestCommand;
        assert_eq!(cmd.name(), "test");
        assert_eq!(cmd.description(), "A test command");
    }
}
