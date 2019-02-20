use dirs;
use std::path::PathBuf;

fn get_default_base_path() -> PathBuf {
    let mut base_path = PathBuf::from("~");
    let home_opt = dirs::home_dir();
    if home_opt.is_some() {
        base_path = home_opt.unwrap();
    }
    base_path.push("Projects");
    base_path.push("git");
    base_path
}

fn get_default_domain() -> &'static str {
    "github.com"
}

fn default_is_https() -> bool {
    false
}

pub fn get_base_path() -> PathBuf {
    return get_default_base_path();
}

pub fn get_is_https() -> bool {
    return default_is_https();
}

pub fn get_domain() -> String {
    return get_default_domain().to_string();
}
