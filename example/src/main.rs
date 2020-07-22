use open_file;

fn main() {
    open_file::open(String::from("Cargo.toml"), Some(String::from("kate")));
}
