//! Form processing and validation for Reinhardt

pub mod bound_field;
pub mod csrf;
pub mod field;
pub mod fields;
pub mod file_handling;
pub mod form;
pub mod formset;
pub mod media;
pub mod model_form;
pub mod model_formset;
pub mod security;
pub mod wizard;

pub use bound_field::BoundField;
pub use csrf::CsrfToken;
pub use field::{
    BooleanField,
    CharField,
    EmailField,
    ErrorType,
    FieldError,
    FieldResult,
    FormField as Field, // Alias for compatibility
    FormField,
    IntegerField,
    Widget,
};
pub use fields::{
    ChoiceField, ComboField, DateField, DateTimeField, DecimalField, DurationField, FileField,
    FloatField, GenericIPAddressField, IPProtocol, ImageField, JSONField, ModelChoiceField,
    ModelMultipleChoiceField, MultiValueField, MultipleChoiceField, RegexField, SlugField,
    SplitDateTimeField, TimeField, URLField, UUIDField,
};
pub use form::{Form, FormError, FormResult};
pub use formset::FormSet;
pub use media::{Media, MediaDefiningWidget};
pub use model_form::{FieldType, FormModel, ModelForm, ModelFormBuilder, ModelFormConfig};
pub use model_formset::{ModelFormSet, ModelFormSetBuilder, ModelFormSetConfig};
pub use wizard::{FormWizard, WizardStep};

// Re-export items from empty modules as they're implemented
// pub use file_handling::{...};
// pub use security::{...};
