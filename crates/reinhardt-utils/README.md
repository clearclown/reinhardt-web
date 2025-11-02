# reinhardt-utils

Common utilities and helper functions

## Overview

Collection of utility functions and helpers used throughout the framework.

Includes date/time utilities, string manipulation, encoding/decoding, and other common operations.

## Features

### Implemented ✓

#### HTML Utilities (`html` module)

- **HTML Escaping/Unescaping**
  - `escape()`: Escapes HTML special characters (`<`, `>`, `&`, `"`, `'`)
  - `unescape()`: Unescapes HTML entities
  - `conditional_escape()`: Conditional escaping with autoescape flag support
  - `escape_attr()`: HTML attribute value escaping (handles newlines and tabs)
- **HTML Manipulation**
  - `strip_tags()`: Removes HTML tags
  - `strip_spaces_between_tags()`: Removes whitespace between tags
  - `truncate_html_words()`: Truncates by word count while preserving HTML tags
  - `format_html()`: HTML generation via placeholder replacement
- **Safe Strings**
  - `SafeString`: Safe string type for bypassing automatic escaping

#### Encoding Utilities (`encoding` module)

- **URL Encoding**
  - `urlencode()`: URL encoding (spaces converted to `+`)
  - `urldecode()`: URL decoding
- **JavaScript Escaping**
  - `escapejs()`: JavaScript string escaping (handles quotes, control characters, special characters)
- **Slugification**
  - `slugify()`: URL slug generation (lowercase, special character removal, hyphen-separated)
- **Text Processing**
  - `truncate_chars()`: Truncate by character count (appends `...`)
  - `truncate_words()`: Truncate by word count (appends `...`)
  - `wrap_text()`: Wrap text at specified width
  - `force_str()`: Safely convert byte sequences to UTF-8 strings
  - `force_bytes()`: Convert strings to byte sequences
- **Line Break Processing**
  - `linebreaks()`: Convert line breaks to `<br>` tags (with paragraph support)
  - `linebreaksbr()`: Convert line breaks to `<br>` tags (simple version)

#### Date/Time Formatting (`dateformat` module)

- **Django/PHP-style Formatting**
  - `format()`: Date/time formatting with format strings
  - Supported format codes:
    - Year: `Y` (4-digit), `y` (2-digit)
    - Month: `m` (zero-padded), `n` (no padding), `F` (full name), `M` (abbreviated)
    - Day: `d` (zero-padded), `j` (no padding), `l` (day name), `D` (day abbreviated)
    - Hour: `H` (24-hour), `h` (12-hour), `G`/`g` (unpadded versions)
    - Minute: `i`, Second: `s`
    - AM/PM: `A` (uppercase), `a` (lowercase)
- **Shortcut Functions** (`shortcuts` submodule)
  - `iso_date()`: YYYY-MM-DD format
  - `iso_datetime()`: YYYY-MM-DD HH:MM:SS format
  - `us_date()`: MM/DD/YYYY format
  - `eu_date()`: DD/MM/YYYY format
  - `full_date()`: "Monday, January 1, 2025" format
  - `short_date()`: "Jan 1, 2025" format
  - `time_24()`: 24-hour format time
  - `time_12()`: 12-hour format time (with AM/PM)

#### Text Manipulation (`text` module)

- **Case Conversion**
  - `capfirst()`: Capitalize first letter of each word
  - `title()`: Title case conversion (first letter uppercase, rest lowercase)
- **Number Formatting**
  - `intcomma()`: Add comma separators to integers (every 3 digits)
  - `floatcomma()`: Add comma separators to floating-point numbers
  - `ordinal()`: Add ordinal suffixes (1st, 2nd, 3rd, 4th, etc.)
- **Singular/Plural**
  - `pluralize()`: Toggle singular/plural based on count
- **Padding**
  - `ljust()`: Left-justify (right padding)
  - `rjust()`: Right-justify (left padding)
  - `center()`: Center-align (both-side padding)
- **Phone Number Formatting**
  - `phone_format()`: Convert 10/11-digit phone numbers to `(XXX) XXX-XXXX` format

#### Timezone Utilities (`timezone` module)

- **Basic DateTime Retrieval**
  - `now()`: Current UTC time
  - `localtime()`: Current local time
- **Timezone Conversion**
  - `to_local()`: UTC to local timezone conversion
  - `to_utc()`: Local to UTC conversion
  - `to_timezone()`: Timezone conversion by IANA name (currently UTC only)
- **Naive/Aware Conversion**
  - `make_aware_utc()`: Convert naive datetime to UTC timezone-aware
  - `make_aware_local()`: Convert naive datetime to local timezone-aware
  - `is_aware()`: Check for timezone information presence (always `true` in Rust)
- **Parse/Format**
  - `parse_datetime()`: Parse ISO 8601 datetime strings
  - `format_datetime()`: Output datetime in ISO 8601 format (RFC 3339)
- **Timezone Name Retrieval**
  - `get_timezone_name_utc()`: Get timezone name for UTC datetime
  - `get_timezone_name_local()`: Get timezone name for local datetime
