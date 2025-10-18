//! Reinhardt Admin
//!
//! Global command-line tool for the Reinhardt web framework (django-admin equivalent).
//! Used for project-independent tasks like creating new projects and apps.

use clap::{Parser, Subcommand};
use reinhardt_commands::{BaseCommand, CommandContext, StartAppCommand, StartProjectCommand};

#[derive(Parser)]
#[command(name = "reinhardt-admin")]
#[command(about = "Reinhardt web framework global command-line tool", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Reinhardt project
    Startproject {
        /// Project name
        name: String,

        /// Target directory (optional, defaults to current directory)
        #[arg(short, long)]
        directory: Option<String>,
    },

    /// Create a new Reinhardt app
    Startapp {
        /// App name
        name: String,

        /// Target directory (optional, defaults to current directory)
        #[arg(short, long)]
        directory: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Startproject { name, directory } => {
            let cmd = StartProjectCommand;
            let mut args = vec![name];
            if let Some(dir) = directory {
                args.push(dir);
            }
            let ctx = CommandContext::new(args);
            cmd.execute(&ctx).await?;
        }
        Commands::Startapp { name, directory } => {
            let cmd = StartAppCommand;
            let mut args = vec![name];
            if let Some(dir) = directory {
                args.push(dir);
            }
            let ctx = CommandContext::new(args);
            cmd.execute(&ctx).await?;
        }
    }

    Ok(())
}
