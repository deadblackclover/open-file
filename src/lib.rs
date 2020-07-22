//! Opening file in editor
//!
//! ## Usage
//! ```rust
//! extern crate open_file;
//!
//! fn main() {
//!     open_file::open(String::from("Cargo.toml"), Some(String::from("kate")));
//! }
//! ```
use std::env;
use std::process::Command;

/// Function opens the file in the editor
pub fn open(file: String, opts: Option<String>) {
    let application = editor(opts);

    Command::new(&application)
        .arg(file)
        .output()
        .expect("failed to execute process");
}

/// Function returns the default editor depending on the operating system
fn default_editor() -> String {
    if env::consts::OS == "windows" {
        String::from("notepad")
    } else {
        String::from("vi")
    }
}

/// Function returns the editor
fn editor(opts: Option<String>) -> String {
    let visual = match env::var("VISUAL") {
        Ok(env) => env,
        Err(_) => String::new(),
    };

    let editor = match env::var("EDITOR") {
        Ok(env) => env,
        Err(_) => String::new(),
    };

    let default = default_editor();

    if !opts.is_none() {
        opts.unwrap()
    } else if !visual.is_empty() {
        visual
    } else if !editor.is_empty() {
        editor
    } else {
        default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_family = "unix")]
    fn test_default_unix() {
        assert_eq!(default_editor(), String::from("vi"));
    }

    #[test]
    #[cfg(target_family = "windows")]
    fn test_default_windows() {
        assert_eq!(default_editor(), String::from("notepad"));
    }

    #[test]
    fn test_editor() {
        assert_eq!(editor(Some(String::from("nano"))), String::from("nano"));
    }

    #[test]
    fn test_editor_env() {
        env::set_var("EDITOR", "nano");
        assert_eq!(editor(None), String::from("nano"));
    }
}
