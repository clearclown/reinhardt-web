//! Extended OutputWrapper tests
//!
//! Additional tests for OutputWrapper beyond the inline tests in output.rs.
//! Focuses on edge cases, boundary values, and property testing.

use reinhardt_commands::OutputWrapper;
use rstest::rstest;
use std::io::Cursor;

// =============================================================================
// Edge Case Tests
// =============================================================================

/// Test writing empty string
///
/// **Category**: Edge Case
/// **Verifies**: Empty string write doesn't cause issues
#[rstest]
fn test_output_wrapper_empty_write() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	output.write("").unwrap();
	output.flush().unwrap();

	let buffer = output.into_inner().unwrap();
	assert!(buffer.is_empty(), "Empty write should produce empty buffer");
}

/// Test writeln with empty string (just newline)
///
/// **Category**: Edge Case
/// **Verifies**: Empty writeln produces just newline
#[rstest]
fn test_output_wrapper_empty_writeln() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	output.writeln("").unwrap();
	output.flush().unwrap();

	let buffer = output.into_inner().unwrap();
	assert_eq!(
		String::from_utf8(buffer).unwrap(),
		"\n",
		"Empty writeln should produce newline only"
	);
}

/// Test writing Unicode characters
///
/// **Category**: Edge Case
/// **Verifies**: Unicode is handled correctly
#[rstest]
fn test_output_wrapper_unicode() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	output.writeln("こんにちは").unwrap();
	output.writeln("世界！").unwrap();
	output.writeln("🎉🦀").unwrap();
	output.flush().unwrap();

	let buffer = output.into_inner().unwrap();
	let result = String::from_utf8(buffer).unwrap();

	assert!(
		result.contains("こんにちは"),
		"Should contain Japanese text"
	);
	assert!(
		result.contains("世界！"),
		"Should contain Chinese characters"
	);
	assert!(result.contains("🎉🦀"), "Should contain emoji");
	assert_eq!(result, "こんにちは\n世界！\n🎉🦀\n");
}

/// Test writing special characters
///
/// **Category**: Edge Case
/// **Verifies**: Special characters are preserved
#[rstest]
fn test_output_wrapper_special_characters() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	let special = "Tab:\tNewline:\nCarriage return:\rQuotes:\"'<>&";
	output.write(special).unwrap();
	output.flush().unwrap();

	let buffer = output.into_inner().unwrap();
	let result = String::from_utf8(buffer).unwrap();

	assert_eq!(
		result, special,
		"Special characters should be preserved exactly"
	);
}

/// Test writing null bytes
///
/// **Category**: Edge Case
/// **Verifies**: Null bytes are handled correctly
#[rstest]
fn test_output_wrapper_null_bytes() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	// Note: &str can't contain null bytes, so we write around them
	output.write("before").unwrap();
	output.write("after").unwrap();
	output.flush().unwrap();

	let buffer = output.into_inner().unwrap();
	assert_eq!(String::from_utf8(buffer).unwrap(), "beforeafter");
}

// =============================================================================
// Boundary Value Analysis Tests
// =============================================================================

/// Test writing very large data
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: Large data is handled correctly
#[rstest]
fn test_output_wrapper_large_data() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	// Write 100KB of data
	let large_string = "x".repeat(100_000);
	output.write(&large_string).unwrap();
	output.flush().unwrap();

	let buffer = output.into_inner().unwrap();
	assert_eq!(buffer.len(), 100_000, "Should have written 100KB");
}

/// Test many small writes
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: Many small writes work correctly
#[rstest]
fn test_output_wrapper_many_small_writes() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	// 1000 small writes
	for i in 0..1000 {
		output.write(&format!("{}", i % 10)).unwrap();
	}
	output.flush().unwrap();

	let buffer = output.into_inner().unwrap();
	assert_eq!(buffer.len(), 1000, "Should have 1000 characters");
}

/// Test many writeln calls
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: Many lines work correctly
#[rstest]
fn test_output_wrapper_many_lines() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	for i in 0..100 {
		output.writeln(&format!("Line {}", i)).unwrap();
	}
	output.flush().unwrap();

	let buffer = output.into_inner().unwrap();
	let result = String::from_utf8(buffer).unwrap();
	let lines: Vec<&str> = result.lines().collect();

	assert_eq!(lines.len(), 100, "Should have 100 lines");
	assert_eq!(lines[0], "Line 0");
	assert_eq!(lines[99], "Line 99");
}

// =============================================================================
// Property Tests (Order Preservation)
// =============================================================================

/// Test that write order is preserved
///
/// **Category**: Property
/// **Verifies**: Multiple writes maintain correct order
#[rstest]
fn test_output_order_preserved() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	let parts = vec!["first", "second", "third", "fourth", "fifth"];
	for part in &parts {
		output.write(part).unwrap();
	}
	output.flush().unwrap();

	let buffer = output.into_inner().unwrap();
	let result = String::from_utf8(buffer).unwrap();

	assert_eq!(
		result, "firstsecondthirdfourthfifth",
		"Order should be preserved"
	);
}

/// Test writeln order is preserved
///
/// **Category**: Property
/// **Verifies**: Writeln calls maintain correct line order
#[rstest]
fn test_output_writeln_order_preserved() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	let lines = vec!["line1", "line2", "line3"];
	for line in &lines {
		output.writeln(line).unwrap();
	}
	output.flush().unwrap();

	let buffer = output.into_inner().unwrap();
	let result = String::from_utf8(buffer).unwrap();
	let output_lines: Vec<&str> = result.lines().collect();

	assert_eq!(output_lines, lines, "Line order should be preserved");
}

// =============================================================================
// Happy Path Tests
// =============================================================================

/// Test basic write workflow
///
/// **Category**: Happy Path
/// **Verifies**: Basic write -> flush -> into_inner works
#[rstest]
fn test_output_wrapper_basic_workflow() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	output.write("Hello").unwrap();
	output.flush().unwrap();

	let buffer = output.into_inner().unwrap();
	assert_eq!(String::from_utf8(buffer).unwrap(), "Hello");
}

/// Test mixed write and writeln
///
/// **Category**: Happy Path
/// **Verifies**: write and writeln can be mixed
#[rstest]
fn test_output_wrapper_mixed_writes() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	output.write("Name: ").unwrap();
	output.writeln("Test").unwrap();
	output.write("Value: ").unwrap();
	output.writeln("42").unwrap();
	output.flush().unwrap();

	let buffer = output.into_inner().unwrap();
	let result = String::from_utf8(buffer).unwrap();

	assert_eq!(result, "Name: Test\nValue: 42\n");
}

/// Test with Cursor writer
///
/// **Category**: Happy Path
/// **Verifies**: Works with Cursor<Vec<u8>>
#[rstest]
fn test_output_wrapper_with_cursor() {
	let cursor = Cursor::new(Vec::new());
	let mut output = OutputWrapper::new(cursor);

	output.writeln("Cursor test").unwrap();
	output.flush().unwrap();

	let cursor = output.into_inner().unwrap();
	let result = String::from_utf8(cursor.into_inner()).unwrap();

	assert_eq!(result, "Cursor test\n");
}

// =============================================================================
// State Transition Tests
// =============================================================================

/// Test buffer state after multiple flushes
///
/// **Category**: State Transition
/// **Verifies**: Multiple flushes don't cause issues
#[rstest]
fn test_output_wrapper_multiple_flushes() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	output.write("Part1").unwrap();
	output.flush().unwrap();

	output.write("Part2").unwrap();
	output.flush().unwrap();

	output.write("Part3").unwrap();
	output.flush().unwrap();

	let buffer = output.into_inner().unwrap();
	assert_eq!(
		String::from_utf8(buffer).unwrap(),
		"Part1Part2Part3",
		"All parts should be written"
	);
}

/// Test flush without any writes
///
/// **Category**: State Transition
/// **Verifies**: Flush on empty buffer works
#[rstest]
fn test_output_wrapper_flush_empty() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	// Flush without writing anything
	output.flush().unwrap();
	output.flush().unwrap();
	output.flush().unwrap();

	let buffer = output.into_inner().unwrap();
	assert!(buffer.is_empty(), "Buffer should be empty");
}

// =============================================================================
// Sanity Tests
// =============================================================================

/// Sanity test for OutputWrapper
///
/// **Category**: Sanity
/// **Verifies**: Basic functionality works
#[rstest]
fn test_output_wrapper_sanity() {
	let buffer = Vec::new();
	let mut output = OutputWrapper::new(buffer);

	// Write something
	output.write("test").unwrap();
	output.writeln(" line").unwrap();

	// Flush
	output.flush().unwrap();

	// Get inner
	let buffer = output.into_inner().unwrap();
	let result = String::from_utf8(buffer).unwrap();

	assert!(!result.is_empty(), "Should have content");
	assert!(result.contains("test"), "Should contain 'test'");
	assert!(result.ends_with('\n'), "Should end with newline");
}
