//! Output formatting utilities

use colored::Colorize;
use serde::Serialize;
use serde_json::Value;

/// Output format for displaying values
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
	Text,
	Json,
	Toml,
}

// Note: OutputFormat::parse() was removed as clap's ValueEnum is used instead
// See OutputFormatArg in commands/show.rs
/// Print a success message
///
pub fn success(msg: &str) {
	println!("{} {}", "✓".green().bold(), msg);
}
/// Print an error message
///
pub fn error(msg: &str) {
	eprintln!("{} {}", "✗".red().bold(), msg);
}
/// Print a warning message
///
pub fn warning(msg: &str) {
	println!("{} {}", "⚠".yellow().bold(), msg);
}
/// Print an info message
///
pub fn info(msg: &str) {
	println!("{} {}", "ℹ".blue().bold(), msg);
}
// Note: key_value(), table_header(), and table_row() were removed as unused
// These were placeholder utility functions that were never integrated into any command
/// Format and print a value based on the output format
///
pub fn print_value<T: Serialize>(value: &T, format: OutputFormat) -> anyhow::Result<()> {
	match format {
		OutputFormat::Json => {
			let json = serde_json::to_string_pretty(value)?;
			println!("{}", json);
		}
		OutputFormat::Toml => {
			let toml_str = toml::to_string_pretty(value)?;
			println!("{}", toml_str);
		}
		OutputFormat::Text => {
			// For text format, try to convert to a pretty-printed JSON first
			let json = serde_json::to_value(value)?;
			print_value_text(&json, 0);
		}
	}
	Ok(())
}

fn print_value_text(value: &Value, indent: usize) {
	let indent_str = "  ".repeat(indent);
	match value {
		Value::Object(map) => {
			for (key, val) in map {
				match val {
					Value::Object(_) | Value::Array(_) => {
						println!("{}{}:", indent_str, key.cyan().bold());
						print_value_text(val, indent + 1);
					}
					_ => {
						print!("{}{}: ", indent_str, key.cyan().bold());
						print_value_text(val, 0);
					}
				}
			}
		}
		Value::Array(arr) => {
			for val in arr {
				print!("{}- ", indent_str);
				print_value_text(val, indent + 1);
			}
		}
		Value::String(s) => println!("{}", s.green()),
		Value::Number(n) => println!("{}", n.to_string().yellow()),
		Value::Bool(b) => println!("{}", b.to_string().blue()),
		Value::Null => println!("{}", "null".dimmed()),
	}
}
/// Print a diff between two values
///
pub fn print_diff(key: &str, old_value: Option<&str>, new_value: Option<&str>) {
	match (old_value, new_value) {
		(Some(old), Some(new)) if old != new => {
			println!(
				"{} {} {} → {}",
				"~".yellow().bold(),
				key.cyan(),
				old.red().strikethrough(),
				new.green()
			);
		}
		(None, Some(new)) => {
			println!("{} {} {}", "+".green().bold(), key.cyan(), new.green());
		}
		(Some(old), None) => {
			println!(
				"{} {} {}",
				"-".red().bold(),
				key.cyan(),
				old.red().strikethrough()
			);
		}
		_ => {}
	}
}
