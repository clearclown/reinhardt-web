# Validation Messages - Japanese (日本語)
# These messages are used by reinhardt-validators for localized error messages.

# String validators
validation-too-short = 値が短すぎます: { $length }文字 (最小: { $min }文字)
validation-too-long = 値が長すぎます: { $length }文字 (最大: { $max }文字)
validation-pattern-mismatch = 値が必要なパターンに一致しません
validation-invalid-slug = スラッグの形式が無効です: { $value }
validation-invalid-uuid = UUIDの形式が無効です: { $value }

# Numeric validators
validation-too-small = 値が小さすぎます: { $value } (最小: { $min })
validation-too-large = 値が大きすぎます: { $value } (最大: { $max })
validation-out-of-range = 値は { $min } から { $max } の間である必要があります

# Email validator
validation-invalid-email = メールアドレスが無効です: { $value }

# URL validator
validation-invalid-url = URLが無効です: { $value }

# IP address validator
validation-invalid-ip = IPアドレスが無効です: { $value }
validation-ip-wrong-version = { $expected } アドレスが期待されましたが、{ $actual } が入力されました

# Date/Time validators
validation-invalid-date = 日付の形式が無効です: { $value }
validation-invalid-time = 時刻の形式が無効です: { $value }
validation-invalid-datetime = 日時の形式が無効です: { $value }

# JSON validator
validation-invalid-json = JSONが無効です: { $error }

# Credit card validator
validation-invalid-credit-card = クレジットカード番号が無効です
validation-card-type-not-allowed = カードタイプ { $card_type } は許可されていません (許可: { $allowed })

# Phone number validator
validation-invalid-phone = 電話番号が無効です: { $value }
validation-country-not-allowed = 国コード { $country } は許可されていません (許可: { $allowed })

# IBAN validator
validation-invalid-iban = IBANが無効です: { $value }
validation-iban-country-not-allowed = IBAN国 { $country } は許可されていません (許可: { $allowed })

# File validators
validation-invalid-extension = ファイル拡張子 "{ $extension }" は許可されていません (許可: { $allowed })
validation-invalid-mime-type = MIMEタイプ "{ $mime_type }" は許可されていません (許可: { $allowed })
validation-file-too-small = ファイルが小さすぎます: { $size }バイト (最小: { $min }バイト)
validation-file-too-large = ファイルが大きすぎます: { $size }バイト (最大: { $max }バイト)

# Image validators
validation-image-width-too-small = 画像の幅が小さすぎます: { $width }px (最小: { $min }px)
validation-image-width-too-large = 画像の幅が大きすぎます: { $width }px (最大: { $max }px)
validation-image-height-too-small = 画像の高さが小さすぎます: { $height }px (最小: { $min }px)
validation-image-height-too-large = 画像の高さが大きすぎます: { $height }px (最大: { $max }px)
validation-invalid-aspect-ratio = アスペクト比が無効です: { $actual_width }:{ $actual_height } (期待: { $expected_width }:{ $expected_height })
validation-image-read-error = 画像を読み込めません: { $error }

# Postal code validator
validation-invalid-postal-code = 郵便番号が無効です: { $value }
validation-postal-country-not-recognized = 郵便番号の国が認識できません: { $value }
validation-postal-country-not-allowed = 国 { $country } は許可されていません (許可: { $allowed })

# Color validator
validation-invalid-color = 色の形式が無効です: { $value }

# Uniqueness validators
validation-not-unique = 値は一意である必要があります。"{ $value }" はフィールド "{ $field }" に既に存在します

# Foreign key validator
validation-fk-not-found = 参照が見つかりません: フィールド { $field } の値 { $value } は { $table } に存在しません

# Composite validators
validation-all-failed = すべてのバリデータが失敗しました: { $errors }
validation-composite-failed = 検証に失敗しました: { $error }

# Generic
validation-custom = { $message }
