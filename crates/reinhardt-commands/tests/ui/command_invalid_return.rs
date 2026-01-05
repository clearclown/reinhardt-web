//! Invalid command - wrong return type for execute
//!
//! This file should fail to compile with an error about incompatible return type.

use async_trait::async_trait;
use reinhardt_commands::{
	BaseCommand, CommandArgument, CommandContext, CommandError, CommandOption,
};

/// Invalid command with wrong return type
pub struct InvalidReturnCommand;

#[async_trait]
impl BaseCommand for InvalidReturnCommand {
	fn name(&self) -> &str {
		"invalid_return"
	}

	fn description(&self) -> &str {
		"Command with invalid return type"
	}

	fn arguments(&self) -> Vec<CommandArgument> {
		vec![]
	}

	fn options(&self) -> Vec<CommandOption> {
		vec![]
	}

	// Wrong return type: should be Result<(), CommandError>
	async fn execute(&self, _ctx: &CommandContext) -> String {
		"wrong return type".to_string()
	}
}

fn main() {}
