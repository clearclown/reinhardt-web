//! Collect static files command

use crate::CommandResult;
use crate::{BaseCommand, CommandContext};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct CollectStaticOptions {
    pub clear: bool,
    pub no_input: bool,
    pub dry_run: bool,
    pub interactive: bool,
    pub verbosity: u8,
    pub link: bool,
    pub ignore_patterns: Vec<String>,
}

impl Default for CollectStaticOptions {
    fn default() -> Self {
        Self {
            clear: false,
            no_input: false,
            dry_run: false,
            interactive: true,
            verbosity: 1,
            link: false,
            ignore_patterns: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CollectStaticStats {
    pub copied: usize,
    pub skipped: usize,
    pub deleted: usize,
}

impl CollectStaticStats {
    pub fn new() -> Self {
        Self {
            copied: 0,
            skipped: 0,
            deleted: 0,
        }
    }
}

impl Default for CollectStaticStats {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CollectStaticCommand {
    #[allow(dead_code)]
    options: CollectStaticOptions,
}

impl CollectStaticCommand {
    pub fn new(options: CollectStaticOptions) -> Self {
        Self { options }
    }
}

#[async_trait]
impl BaseCommand for CollectStaticCommand {
    fn name(&self) -> &str {
        "collectstatic"
    }

    async fn execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
        Ok(())
    }
}
