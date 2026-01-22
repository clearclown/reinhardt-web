# Validation Messages - English
# These messages are used by reinhardt-validators for localized error messages.

# String validators
validation-too-short = Value is too short: { $length } characters (minimum: { $min })
validation-too-long = Value is too long: { $length } characters (maximum: { $max })
validation-pattern-mismatch = Value does not match the required pattern
validation-invalid-slug = Invalid slug format: { $value }
validation-invalid-uuid = Invalid UUID format: { $value }

# Numeric validators
validation-too-small = Value is too small: { $value } (minimum: { $min })
validation-too-large = Value is too large: { $value } (maximum: { $max })
validation-out-of-range = Value must be between { $min } and { $max }

# Email validator
validation-invalid-email = Invalid email address: { $value }

# URL validator
validation-invalid-url = Invalid URL: { $value }

# IP address validator
validation-invalid-ip = Invalid IP address: { $value }
validation-ip-wrong-version = Expected { $expected } address, got { $actual }

# Date/Time validators
validation-invalid-date = Invalid date format: { $value }
validation-invalid-time = Invalid time format: { $value }
validation-invalid-datetime = Invalid datetime format: { $value }

# JSON validator
validation-invalid-json = Invalid JSON: { $error }

# Credit card validator
validation-invalid-credit-card = Invalid credit card number
validation-card-type-not-allowed = Card type { $card_type } is not allowed (allowed: { $allowed })

# Phone number validator
validation-invalid-phone = Invalid phone number: { $value }
validation-country-not-allowed = Country code { $country } is not allowed (allowed: { $allowed })

# IBAN validator
validation-invalid-iban = Invalid IBAN: { $value }
validation-iban-country-not-allowed = IBAN country { $country } is not allowed (allowed: { $allowed })

# File validators
validation-invalid-extension = File extension "{ $extension }" is not allowed (allowed: { $allowed })
validation-invalid-mime-type = MIME type "{ $mime_type }" is not allowed (allowed: { $allowed })
validation-file-too-small = File is too small: { $size } bytes (minimum: { $min } bytes)
validation-file-too-large = File is too large: { $size } bytes (maximum: { $max } bytes)

# Image validators
validation-image-width-too-small = Image width is too small: { $width }px (minimum: { $min }px)
validation-image-width-too-large = Image width is too large: { $width }px (maximum: { $max }px)
validation-image-height-too-small = Image height is too small: { $height }px (minimum: { $min }px)
validation-image-height-too-large = Image height is too large: { $height }px (maximum: { $max }px)
validation-invalid-aspect-ratio = Invalid aspect ratio: { $actual_width }:{ $actual_height } (expected: { $expected_width }:{ $expected_height })
validation-image-read-error = Cannot read image: { $error }

# Postal code validator
validation-invalid-postal-code = Invalid postal code: { $value }
validation-postal-country-not-recognized = Postal code country not recognized: { $value }
validation-postal-country-not-allowed = Country { $country } is not allowed (allowed: { $allowed })

# Color validator
validation-invalid-color = Invalid color format: { $value }

# Uniqueness validators
validation-not-unique = Value must be unique. "{ $value }" already exists in field "{ $field }"

# Foreign key validator
validation-fk-not-found = Reference not found: { $field } with value { $value } does not exist in { $table }

# Composite validators
validation-all-failed = All validators failed: { $errors }
validation-composite-failed = Validation failed: { $error }

# Generic
validation-custom = { $message }
