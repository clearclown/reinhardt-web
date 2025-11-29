//! Field type definitions for migrations

use serde::{Deserialize, Serialize};

/// データベースフィールドタイプを表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FieldType {
	BigInteger,
	Integer,
	SmallInteger,
	Char {
		max_length: usize,
	},
	Text,
	DateTime,
	Date,
	Time,
	Boolean,
	Decimal {
		max_digits: usize,
		decimal_places: usize,
	},
	Binary,
	Json,
	Uuid,
	Custom(String),
}

impl FieldType {
	/// 文字列表現から変換（後方互換性）
	pub fn from_string(s: &str) -> Self {
		match s {
			"BigIntegerField" => FieldType::BigInteger,
			"IntegerField" => FieldType::Integer,
			"SmallIntegerField" => FieldType::SmallInteger,
			"CharField" => FieldType::Char { max_length: 255 },
			"TextField" => FieldType::Text,
			"DateTimeField" => FieldType::DateTime,
			"DateField" => FieldType::Date,
			"TimeField" => FieldType::Time,
			"BooleanField" => FieldType::Boolean,
			"UUIDField" => FieldType::Uuid,
			"JSONField" => FieldType::Json,
			_ => FieldType::Custom(s.to_string()),
		}
	}

	/// フィールド名表現に変換
	pub fn to_field_name(&self) -> &'static str {
		match self {
			FieldType::BigInteger => "BigIntegerField",
			FieldType::Integer => "IntegerField",
			FieldType::SmallInteger => "SmallIntegerField",
			FieldType::Char { .. } => "CharField",
			FieldType::Text => "TextField",
			FieldType::DateTime => "DateTimeField",
			FieldType::Date => "DateField",
			FieldType::Time => "TimeField",
			FieldType::Boolean => "BooleanField",
			FieldType::Decimal { .. } => "DecimalField",
			FieldType::Binary => "BinaryField",
			FieldType::Json => "JSONField",
			FieldType::Uuid => "UUIDField",
			FieldType::Custom(_) => "CustomField",
		}
	}
}

/// Trait for field types that provide their type name as a compile-time constant
pub trait FieldTypeName {
	const NAME: &'static str;
}

// Type-safe field type markers
pub struct BigIntegerField;
impl FieldTypeName for BigIntegerField {
	const NAME: &'static str = "BigIntegerField";
}

pub struct IntegerField;
impl FieldTypeName for IntegerField {
	const NAME: &'static str = "IntegerField";
}

pub struct SmallIntegerField;
impl FieldTypeName for SmallIntegerField {
	const NAME: &'static str = "SmallIntegerField";
}

pub struct CharField;
impl FieldTypeName for CharField {
	const NAME: &'static str = "CharField";
}

pub struct TextField;
impl FieldTypeName for TextField {
	const NAME: &'static str = "TextField";
}

pub struct DateTimeField;
impl FieldTypeName for DateTimeField {
	const NAME: &'static str = "DateTimeField";
}

pub struct DateField;
impl FieldTypeName for DateField {
	const NAME: &'static str = "DateField";
}

pub struct TimeField;
impl FieldTypeName for TimeField {
	const NAME: &'static str = "TimeField";
}

pub struct BooleanField;
impl FieldTypeName for BooleanField {
	const NAME: &'static str = "BooleanField";
}

pub struct DecimalField;
impl FieldTypeName for DecimalField {
	const NAME: &'static str = "DecimalField";
}

pub struct BinaryField;
impl FieldTypeName for BinaryField {
	const NAME: &'static str = "BinaryField";
}

pub struct JSONField;
impl FieldTypeName for JSONField {
	const NAME: &'static str = "JSONField";
}

pub struct UUIDField;
impl FieldTypeName for UUIDField {
	const NAME: &'static str = "UUIDField";
}

pub mod prelude {
	pub use super::{
		BigIntegerField, BinaryField, BooleanField, CharField, DateField, DateTimeField,
		DecimalField, FieldTypeName, IntegerField, JSONField, SmallIntegerField, TextField,
		TimeField, UUIDField,
	};
}
