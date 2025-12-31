//! Invalid command - missing name method
//!
//! This file should fail to compile with an error about missing `name` method.

use reinhardt_commands::{BaseCommand, CommandContext, CommandArgument, CommandOption, CommandError};
use async_trait::async_trait;

/// Invalid command missing the `name` method
pub struct InvalidCommand;

#[async_trait]
impl BaseCommand for InvalidCommand {
	// Missing: fn name(&self) -> &str

	fn description(&self) -> &str {
		"An invalid command"
	}

	fn arguments(&self) -> Vec<CommandArgument> {
		vec![]
	}

	fn options(&self) -> Vec<CommandOption> {
		vec![]
	}

	async fn execute(&self, _ctx: &CommandContext) -> Result<(), CommandError> {
		Ok(())
	}
}

fn main() {}
