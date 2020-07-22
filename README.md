# open-file

![crates.io](https://img.shields.io/crates/v/open-file.svg)
![docs.rs](https://docs.rs/open-file/badge.svg)

Opening file in editor

## Usage
```rust
use open_file;

fn main() {
    open_file::open(String::from("Cargo.toml"), Some(String::from("kate")));
}
```