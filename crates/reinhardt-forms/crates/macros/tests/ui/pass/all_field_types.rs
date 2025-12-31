//! HP-004: All field types.
//!
//! Tests that all supported field types compile successfully.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			// Text fields
			char_field: CharField {},
			email_field: EmailField {},
			url_field: URLField {},
			slug_field: SlugField {},
			regex_field: RegexField {},

			// Numeric fields
			integer_field: IntegerField {},
			float_field: FloatField {},
			decimal_field: DecimalField {},

			// Boolean
			boolean_field: BooleanField {},

			// Date/Time fields
			date_field: DateField {},
			datetime_field: DateTimeField {},
			time_field: TimeField {},
			duration_field: DurationField {},

			// Choice fields
			choice_field: ChoiceField {},
			multiple_choice_field: MultipleChoiceField {},
			model_choice_field: ModelChoiceField {},
			model_multiple_choice_field: ModelMultipleChoiceField {},

			// File fields
			file_field: FileField {},
			image_field: ImageField {},

			// Other fields
			json_field: JSONField {},
			uuid_field: UUIDField {},
			color_field: ColorField {},
			password_field: PasswordField {},
			combo_field: ComboField {},
		},
	};
}
