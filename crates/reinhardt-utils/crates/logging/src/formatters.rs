pub fn escape_control_chars(s: &str) -> String {
    s.to_string()
}
pub trait Formatter {}
pub struct ServerFormatter;
pub struct StandardFormatter;
