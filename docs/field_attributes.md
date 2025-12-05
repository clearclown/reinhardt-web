# Field Attributes Reference

このドキュメントは、`#[field(...)]`マクロで使用可能な全ての属性をリストアップしています。

## 概要

Reinhardtの`#[derive(Model)]`マクロは、フィールドレベルの属性を通じて、データベーススキーマの詳細な制御を提供します。現在、**42個の属性**（既存20個 + 新規実装22個）がサポートされています。

## 属性の分類

### 全DBMS共通
- 基本属性（primary_key, unique, null, default など）
- Generated Columns（generated, generated_stored）
- 照合順序（collate）

### PostgreSQL専用
- Identity列（identity_always, identity_by_default）
- ストレージ最適化（storage, compression）

### MySQL専用
- 自動採番（auto_increment）
- 文字セット（character_set）
- ON UPDATE（on_update_current_timestamp）
- 不可視カラム（invisible）
- 数値型属性（unsigned, zerofill）※非推奨

### SQLite専用
- 自動採番（autoincrement）
- テーブルレベル属性（strict, without_rowid）

### 複数DBMS対応
- generated_virtual（MySQL, SQLite）
- comment（PostgreSQL, MySQL）
- fulltext（PostgreSQL, MySQL）

---

## 既存の属性（20個）

### 基本制約

#### `primary_key: bool`
プライマリキーを指定します。

```rust
#[field(primary_key = true)]
id: i32,
```

**対応DBMS**: 全て
**SQL出力**: `PRIMARY KEY`

#### `unique: bool`
UNIQUE制約を指定します。

```rust
#[field(unique = true)]
email: String,
```

**対応DBMS**: 全て
**SQL出力**: `UNIQUE`

#### `null: bool`
NULL許可を指定します。デフォルトは`false`（NOT NULL）。

```rust
#[field(null = true)]
optional_field: Option<String>,
```

**対応DBMS**: 全て
**SQL出力**: NULL許可時は何も出力、許可しない時は`NOT NULL`

#### `default: &str`
デフォルト値を指定します。

```rust
#[field(default = "'active'")]
status: String,
```

**対応DBMS**: 全て
**SQL出力**: `DEFAULT 'active'`

#### `db_default: &str`
データベース関数をデフォルト値として指定します。

```rust
#[field(db_default = "CURRENT_TIMESTAMP")]
created_at: chrono::NaiveDateTime,
```

**対応DBMS**: 全て
**SQL出力**: `DEFAULT CURRENT_TIMESTAMP`

### フィールド型・長さ

#### `max_length: usize`
VARCHAR型の最大長を指定します。

```rust
#[field(max_length = 255)]
name: String,
```

**対応DBMS**: 全て
**SQL出力**: `VARCHAR(255)`

#### `min_length: usize`
最小長のバリデーション（アプリケーションレベル）。

```rust
#[field(min_length = 3)]
username: String,
```

**対応DBMS**: 全て（バリデーションのみ）
**SQL出力**: なし

### バリデーション

#### `email: bool`
メールアドレス形式のバリデーションを有効にします。

```rust
#[field(email = true)]
email: String,
```

**対応DBMS**: 全て（バリデーションのみ）
**SQL出力**: なし

#### `url: bool`
URL形式のバリデーションを有効にします。

```rust
#[field(url = true)]
website: String,
```

**対応DBMS**: 全て（バリデーションのみ）
**SQL出力**: なし

#### `min_value: i64`
最小値のバリデーション（アプリケーションレベル）。

```rust
#[field(min_value = 0)]
age: i32,
```

**対応DBMS**: 全て（バリデーションのみ）
**SQL出力**: なし

#### `max_value: i64`
最大値のバリデーション（アプリケーションレベル）。

```rust
#[field(max_value = 150)]
age: i32,
```

**対応DBMS**: 全て（バリデーションのみ）
**SQL出力**: なし

#### `check: &str`
CHECK制約を指定します。

```rust
#[field(check = "age >= 0 AND age <= 150")]
age: i32,
```

**対応DBMS**: 全て
**SQL出力**: `CHECK (age >= 0 AND age <= 150)`

### リレーション

#### `foreign_key: Type or &str`
外部キーを指定します。型名または"app_label.ModelName"形式の文字列。

```rust
#[field(foreign_key = User)]
user_id: i32,

// または
#[field(foreign_key = "users.User")]
user_id: i32,
```

**対応DBMS**: 全て
**SQL出力**: `REFERENCES users(id)`

#### `on_delete: &str`
外部キー削除時の動作を指定します。

値: `"CASCADE"`, `"SET NULL"`, `"RESTRICT"`, `"NO ACTION"`, `"SET DEFAULT"`

```rust
#[field(foreign_key = User, on_delete = "CASCADE")]
user_id: i32,
```

**対応DBMS**: 全て
**SQL出力**: `ON DELETE CASCADE`

### その他

#### `db_column: &str`
データベースカラム名を明示的に指定します。

```rust
#[field(db_column = "user_name")]
username: String,
```

**対応DBMS**: 全て
**SQL出力**: カラム名が`user_name`になる

#### `blank: bool`
空文字列を許可するか（バリデーションレベル）。

```rust
#[field(blank = true)]
description: String,
```

**対応DBMS**: 全て（バリデーションのみ）
**SQL出力**: なし

#### `editable: bool`
フィールドが編集可能かどうか（アプリケーションレベル）。

```rust
#[field(editable = false)]
created_at: chrono::NaiveDateTime,
```

**対応DBMS**: 全て（メタデータのみ）
**SQL出力**: なし

#### `choices: Vec<(Value, Display)>`
選択肢を定義します（アプリケーションレベル）。

```rust
#[field(choices = vec![("active", "Active"), ("inactive", "Inactive")])]
status: String,
```

**対応DBMS**: 全て（バリデーションのみ）
**SQL出力**: なし

#### `help_text: &str`
ヘルプテキスト（ドキュメント用）。

```rust
#[field(help_text = "User's full name")]
name: String,
```

**対応DBMS**: 全て（メタデータのみ）
**SQL出力**: なし

---

## 新規実装属性（22個）

### Phase 1: 標準的かつ複数DBMS共通（10属性）

#### `generated: &str`
**対応DBMS**: 全て
**Feature Flag**: なし

生成列（計算列）の式を指定します。

```rust
#[field(generated = "first_name || ' ' || last_name")]
full_name: String,
```

**SQL出力**:
- PostgreSQL: `GENERATED ALWAYS AS (first_name || ' ' || last_name)`
- MySQL: `GENERATED ALWAYS AS (first_name || ' ' || last_name)`
- SQLite: `AS (first_name || ' ' || last_name)`

#### `generated_stored: bool`
**対応DBMS**: 全て
**Feature Flag**: なし

生成列を物理的に保存するかを指定します。`generated`と併用。

```rust
#[field(generated = "price * quantity", generated_stored = true)]
total: f64,
```

**SQL出力**: `STORED`

#### `generated_virtual: bool`
**対応DBMS**: MySQL, SQLite
**Feature Flag**: `#[cfg(any(feature = "db-mysql", feature = "db-sqlite"))]`

生成列を仮想列として定義します。`generated`と併用。

```rust
#[cfg(any(feature = "db-mysql", feature = "db-sqlite"))]
#[field(generated = "YEAR(birth_date)", generated_virtual = true)]
birth_year: i32,
```

**SQL出力**: `VIRTUAL`

**注意**: PostgreSQLは仮想列をサポートしていません。

#### `identity_always: bool`
**対応DBMS**: PostgreSQL
**Feature Flag**: `#[cfg(feature = "db-postgres")]`

PostgreSQL IDENTITY ALWAYS列を定義します。

```rust
#[cfg(feature = "db-postgres")]
#[field(identity_always = true)]
id: i64,
```

**SQL出力**: `GENERATED ALWAYS AS IDENTITY`

#### `identity_by_default: bool`
**対応DBMS**: PostgreSQL
**Feature Flag**: `#[cfg(feature = "db-postgres")]`

PostgreSQL IDENTITY BY DEFAULT列を定義します。

```rust
#[cfg(feature = "db-postgres")]
#[field(identity_by_default = true)]
id: i64,
```

**SQL出力**: `GENERATED BY DEFAULT AS IDENTITY`

#### `auto_increment: bool`
**対応DBMS**: MySQL
**Feature Flag**: `#[cfg(feature = "db-mysql")]`

MySQL AUTO_INCREMENT属性を指定します。

```rust
#[cfg(feature = "db-mysql")]
#[field(auto_increment = true)]
id: u32,
```

**SQL出力**: `AUTO_INCREMENT`

#### `autoincrement: bool`
**対応DBMS**: SQLite
**Feature Flag**: `#[cfg(feature = "db-sqlite")]`

SQLite AUTOINCREMENT属性を指定します。

```rust
#[cfg(feature = "db-sqlite")]
#[field(primary_key = true, autoincrement = true)]
id: i64,
```

**SQL出力**: `AUTOINCREMENT`

**相互排他**: `identity_always`, `identity_by_default`, `auto_increment`, `autoincrement`は相互排他です。1つのフィールドに複数指定するとコンパイルエラーになります。

#### `collate: &str`
**対応DBMS**: 全て
**Feature Flag**: なし

照合順序を指定します。

```rust
#[field(collate = "utf8mb4_unicode_ci")]
name: String,
```

**SQL出力**: `COLLATE utf8mb4_unicode_ci`

#### `character_set: &str`
**対応DBMS**: MySQL
**Feature Flag**: `#[cfg(feature = "db-mysql")]`

MySQL文字セットを指定します。

```rust
#[cfg(feature = "db-mysql")]
#[field(character_set = "utf8mb4")]
description: String,
```

**SQL出力**: `CHARACTER SET utf8mb4`

#### `comment: &str`
**対応DBMS**: PostgreSQL, MySQL
**Feature Flag**: `#[cfg(any(feature = "db-postgres", feature = "db-mysql"))]`

カラムコメントを指定します。

```rust
#[cfg(any(feature = "db-postgres", feature = "db-mysql"))]
#[field(comment = "User's email address")]
email: String,
```

**SQL出力**:
- PostgreSQL: 別SQL文 `COMMENT ON COLUMN table.column IS 'text'`
- MySQL: `COMMENT 'text'`

---

### Phase 2: 特定DBMSで重要（5属性）

#### `storage: &str`
**対応DBMS**: PostgreSQL
**Feature Flag**: `#[cfg(feature = "db-postgres")]`

PostgreSQLストレージ戦略を指定します。

値: `"plain"`, `"extended"`, `"external"`, `"main"`

```rust
#[cfg(feature = "db-postgres")]
#[field(storage = "external")]
large_text: String,
```

**SQL出力**: `STORAGE EXTERNAL`

**ストレージ戦略の説明**:
- `PLAIN`: インライン保存、圧縮なし
- `EXTENDED`: インライン保存、圧縮あり
- `EXTERNAL`: 別テーブル保存、圧縮なし
- `MAIN`: 別テーブル保存、圧縮あり

#### `compression: &str`
**対応DBMS**: PostgreSQL
**Feature Flag**: `#[cfg(feature = "db-postgres")]`

PostgreSQL圧縮方式を指定します。

値: `"pglz"`, `"lz4"`

```rust
#[cfg(feature = "db-postgres")]
#[field(compression = "lz4")]
data: Vec<u8>,
```

**SQL出力**: `COMPRESSION lz4`

#### `on_update_current_timestamp: bool`
**対応DBMS**: MySQL
**Feature Flag**: `#[cfg(feature = "db-mysql")]`

MySQL ON UPDATE CURRENT_TIMESTAMPを指定します。

```rust
#[cfg(feature = "db-mysql")]
#[field(on_update_current_timestamp = true)]
updated_at: chrono::NaiveDateTime,
```

**SQL出力**: `ON UPDATE CURRENT_TIMESTAMP`

#### `invisible: bool`
**対応DBMS**: MySQL 8.0.23+
**Feature Flag**: `#[cfg(feature = "db-mysql")]`

MySQLカラムを不可視にします。

```rust
#[cfg(feature = "db-mysql")]
#[field(invisible = true)]
internal_metadata: String,
```

**SQL出力**: `INVISIBLE`

**用途**: `SELECT *`で非表示にしたいカラム（監査用メタデータなど）。

#### `fulltext: bool`
**対応DBMS**: PostgreSQL, MySQL
**Feature Flag**: `#[cfg(any(feature = "db-postgres", feature = "db-mysql"))]`

全文検索インデックスを作成する必要があることを示します。

```rust
#[cfg(any(feature = "db-postgres", feature = "db-mysql"))]
#[field(fulltext = true)]
content: String,
```

**SQL出力**: カラム定義には含まれず、別途インデックスとして作成される。

---

### Phase 3: 互換性・特殊用途（4属性）

#### `unsigned: bool` ⚠️ 非推奨
**対応DBMS**: MySQL
**Feature Flag**: `#[cfg(feature = "db-mysql")]`

MySQL符号なし整数型を指定します。

```rust
#[cfg(feature = "db-mysql")]
#[field(unsigned = true)]
count: i32,
```

**SQL出力**: `UNSIGNED`

**⚠️ 注意**: MySQL 8.0.17以降で非推奨。CHECK制約を使用することを推奨。

#### `zerofill: bool` ⚠️ 非推奨
**対応DBMS**: MySQL
**Feature Flag**: `#[cfg(feature = "db-mysql")]`

MySQLゼロ埋め表示を指定します。

```rust
#[cfg(feature = "db-mysql")]
#[field(zerofill = true)]
code: i32,
```

**SQL出力**: `ZEROFILL`

**⚠️ 注意**: MySQL 8.0.17以降で非推奨。アプリケーションレベルでフォーマットすることを推奨。

---

## テーブルレベル属性（`#[model(...)]`）

### `strict: bool`
**対応DBMS**: SQLite
**Feature Flag**: `#[cfg(feature = "db-sqlite")]`

SQLite STRICTテーブルを作成します。

```rust
#[cfg(feature = "db-sqlite")]
#[derive(Model)]
#[model(table_name = "users", strict = true)]
struct User {
	// ...
}
```

**SQL出力**: `CREATE TABLE users (...) STRICT;`

**用途**: 型チェックを厳密にしたい場合。

### `without_rowid: bool`
**対応DBMS**: SQLite
**Feature Flag**: `#[cfg(feature = "db-sqlite")]`

SQLite WITHOUT ROWIDテーブルを作成します。

```rust
#[cfg(feature = "db-sqlite")]
#[derive(Model)]
#[model(table_name = "cache", without_rowid = true)]
struct CacheEntry {
	#[field(primary_key = true)]
	key: String,
	value: String,
}
```

**SQL出力**: `CREATE TABLE cache (...) WITHOUT ROWID;`

**用途**: プライマリキーが整数でない場合のパフォーマンス最適化。

---

## Feature Flags

プロジェクトの`Cargo.toml`で必要なfeature flagsを有効にしてください：

```toml
[dependencies]
reinhardt-macros = { version = "0.1", features = ["db-postgres", "db-mysql", "db-sqlite"] }
reinhardt-migrations = { version = "0.1", features = ["db-postgres", "db-mysql", "db-sqlite"] }
```

利用可能なfeature flags:
- `db-postgres`: PostgreSQL専用属性を有効化
- `db-mysql`: MySQL専用属性を有効化
- `db-sqlite`: SQLite専用属性を有効化

---

## 使用例

### 複雑なモデル例

```rust
use reinhardt::db::orm::prelude::*;
use chrono::NaiveDateTime;

#[derive(Model)]
#[model(table_name = "articles")]
#[cfg_attr(feature = "db-sqlite", model(strict = true))]
struct Article {
	// PostgreSQL: IDENTITY BY DEFAULT
	#[cfg(feature = "db-postgres")]
	#[field(identity_by_default = true)]
	id: i64,

	// MySQL: AUTO_INCREMENT
	#[cfg(feature = "db-mysql")]
	#[field(auto_increment = true)]
	id: u32,

	// SQLite: AUTOINCREMENT
	#[cfg(feature = "db-sqlite")]
	#[field(primary_key = true, autoincrement = true)]
	id: i64,

	// 全DBMS: 基本的な制約
	#[field(max_length = 255, unique = true, collate = "utf8mb4_unicode_ci")]
	title: String,

	// 全文検索対象
	#[cfg(any(feature = "db-postgres", feature = "db-mysql"))]
	#[field(fulltext = true)]
	content: String,

	// 生成列（保存型）
	#[field(generated = "UPPER(title)", generated_stored = true)]
	title_upper: String,

	// コメント付き
	#[cfg(any(feature = "db-postgres", feature = "db-mysql"))]
	#[field(comment = "Article creation timestamp")]
	created_at: NaiveDateTime,

	// MySQL: ON UPDATE CURRENT_TIMESTAMP
	#[cfg(feature = "db-mysql")]
	#[field(on_update_current_timestamp = true)]
	updated_at: NaiveDateTime,

	// PostgreSQL: 圧縮・ストレージ戦略
	#[cfg(feature = "db-postgres")]
	#[field(storage = "external", compression = "lz4")]
	large_data: Vec<u8>,
}
```

---

## マイグレーション

新しい属性を使用すると、マイグレーションシステムが自動的に適切なSQLを生成します：

```bash
# マイグレーションファイルを生成
cargo run --bin manage makemigrations

# マイグレーションを適用
cargo run --bin manage migrate
```

生成されるSQL例（PostgreSQL）:

```sql
CREATE TABLE articles (
	id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
	title VARCHAR(255) UNIQUE COLLATE "utf8mb4_unicode_ci",
	content TEXT,
	title_upper TEXT GENERATED ALWAYS AS (UPPER(title)) STORED,
	created_at TIMESTAMP NOT NULL,
	large_data BYTEA STORAGE EXTERNAL COMPRESSION lz4
);

COMMENT ON COLUMN articles.created_at IS 'Article creation timestamp';
CREATE INDEX idx_articles_content_fulltext ON articles USING GIN (to_tsvector('english', content));
```

---

## トラブルシューティング

### コンパイルエラー: 属性が認識されない

**原因**: 必要なfeature flagが有効になっていない。

**解決策**: `Cargo.toml`で適切なfeature flagを有効にしてください。

### 相互排他エラー: 複数の自動採番属性

```
error: Only one auto-increment attribute can be specified per field
```

**原因**: `identity_always`, `identity_by_default`, `auto_increment`, `autoincrement`のうち、複数が指定されている。

**解決策**: 1つだけを指定してください。

### 生成列とデフォルト値の競合

```
error: Generated columns cannot have default values
```

**原因**: `generated`と`default`が同時に指定されている。

**解決策**: 生成列にはデフォルト値を指定できません。どちらか一方を削除してください。

---

## 参考資料

- [PostgreSQL Generated Columns](https://www.postgresql.org/docs/current/ddl-generated-columns.html)
- [MySQL Generated Columns](https://dev.mysql.com/doc/refman/8.0/en/create-table-generated-columns.html)
- [SQLite Generated Columns](https://www.sqlite.org/gencol.html)
- [PostgreSQL STORAGE](https://www.postgresql.org/docs/current/sql-altertable.html#SQL-ALTERTABLE-DESC-SET-STORAGE)
- [MySQL INVISIBLE Columns](https://dev.mysql.com/doc/refman/8.0/en/invisible-columns.html)
