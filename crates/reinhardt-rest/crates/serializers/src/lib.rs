pub mod content_negotiation;
pub mod fields;
pub mod hyperlinked;
pub mod method_field;
pub mod model_serializer;
pub mod nested;
pub mod parsers;
pub mod relations;
pub mod serializer;
pub mod validator;
pub mod validators;

pub use content_negotiation::ContentNegotiator;
pub use fields::{
    BooleanField, CharField, ChoiceField, EmailField, FieldError, FloatField, IntegerField,
    URLField,
};
pub use model_serializer::ModelSerializer;
pub use serializer::{Deserializer, JsonSerializer, Serializer, SerializerError};
pub use validators::{UniqueTogetherValidator, UniqueValidator};
// pub use hyperlinked::HyperlinkedModelSerializer;
// pub use method_field::SerializerMethodField;
// pub use nested::{ListSerializer, NestedSerializer, WritableNestedSerializer};
// pub use parsers::{FileUploadParser, FormParser, JSONParser, MultiPartParser};
// pub use relations::{
//     HyperlinkedRelatedField, PrimaryKeyRelatedField, SlugRelatedField, StringRelatedField,
// };
// pub use validator::{ValidationError, ValidationResult, Validator};

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        age: i64,
        active: bool,
    }

    // #[test]
    // fn test_json_serializer_serialize() {
    //     let serializer = JsonSerializer::<TestData>::new();
    //     let data = TestData {
    //         name: "Alice".to_string(),
    //         age: 30,
    //         active: true,
    //     };

    //     let result = serializer.serialize(&data).unwrap();
    //     let json_str = String::from_utf8(result).unwrap();

    //     assert!(json_str.contains("Alice"));
    //     assert!(json_str.contains("30"));
    // }

    // #[test]
    // fn test_serializer_char_field_validation() {
    //     use crate::{CharField, Field};
    //     let field = CharField::new().min_length(3).max_length(10);

    //     // Valid string
    //     assert!(field.validate(&"hello".to_string()).is_ok());

    //     // Too short
    //     assert!(field.validate(&"ab".to_string()).is_err());

    //     // Too long
    //     assert!(field.validate(&"this is too long".to_string()).is_err());
    // }

    // #[test]
    // fn test_serializer_integer_field_validation() {
    //     use crate::{Field, IntegerField};
    //     let field = IntegerField::new().min_value(0).max_value(100);

    //     // Valid value
    //     assert!(field.validate(&50).is_ok());

    //     // Too small
    //     assert!(field.validate(&-1).is_err());

    //     // Too large
    //     assert!(field.validate(&101).is_err());
    // }

    // #[test]
    // fn test_email_field_basic_validation() {
    //     use crate::{EmailField, Field};
    //     let field = EmailField::new();

    //     // Valid email
    //     assert!(field.validate(&"test@example.com".to_string()).is_ok());

    //     // Invalid email (no @)
    //     assert!(field.validate(&"notanemail".to_string()).is_err());

    //     // Invalid email (no domain)
    //     assert!(field.validate(&"test@".to_string()).is_err());
    // }

    // #[test]
    // fn test_serializer_validation_error() {
    //     use crate::ValidationError;
    //     let error = ValidationError::new("username", "This field is required");
    //     assert_eq!(error.field, "username");
    //     assert_eq!(error.message, "This field is required");
    // }

    // #[test]
    // fn test_model_serializer_basic() {
    //     use serializer::ModelSerializer as OldModelSerializer;

    //     let validator = |data: &TestData| {
    //         if data.age < 0 {
    //             return Err(vec![ValidationError::new("age", "Age must be positive")]);
    //         }
    //         Ok(())
    //     };

    //     let serializer = OldModelSerializer::new(validator);

    //     // Valid data
    //     let valid_data = TestData {
    //         name: "Dave".to_string(),
    //         age: 40,
    //         active: true,
    //     };
    //     assert!(Serializer::validate(&serializer, &valid_data).is_ok());

    //     // Invalid data
    //     let invalid_data = TestData {
    //         name: "Eve".to_string(),
    //         age: -5,
    //         active: true,
    //     };
    //     assert!(Serializer::validate(&serializer, &invalid_data).is_err());
    // }
}
