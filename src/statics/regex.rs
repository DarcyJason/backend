use once_cell::sync::Lazy;
use regex::Regex;

pub static NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]+$").unwrap());
