[package]
name = "{{ project_name }}"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "manage"
path = "src/bin/manage.rs"

[dependencies]
reinhardt = { version = "*", package = "reinhardt-web", features = ["full"] }

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
