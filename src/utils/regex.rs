use fancy_regex::Regex;
use once_cell::sync::Lazy;

pub static NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]+$").unwrap());
pub static PASSWORD_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)(?=.*[^A-Za-z0-9\s]).+$").unwrap());
