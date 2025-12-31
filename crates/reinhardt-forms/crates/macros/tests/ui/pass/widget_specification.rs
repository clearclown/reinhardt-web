//! HP-011: Widget specification.
//!
//! Tests that widget specifications compile successfully.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			text_input: CharField {
				widget: TextInput,
			},
			textarea: CharField {
				widget: TextArea,
			},
			password: CharField {
				widget: PasswordInput,
			},
			number: IntegerField {
				widget: NumberInput,
			},
			email: EmailField {
				widget: EmailInput,
			},
			url: URLField {
				widget: UrlInput,
			},
			date: DateField {
				widget: DateInput,
			},
			time: TimeField {
				widget: TimeInput,
			},
			datetime: DateTimeField {
				widget: DateTimeInput,
			},
			checkbox: BooleanField {
				widget: Checkbox,
			},
			select: ChoiceField {
				widget: Select,
			},
			radio: ChoiceField {
				widget: RadioSelect,
			},
			checkbox_select: MultipleChoiceField {
				widget: CheckboxSelectMultiple,
			},
			file: FileField {
				widget: FileInput,
			},
			hidden: CharField {
				widget: HiddenInput,
			},
		},
	};
}
