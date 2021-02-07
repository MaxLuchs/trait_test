use std::{fs::File, path::Path};

fn main() {
    println!("Hello, world!");
    File::open(&Path::new("test.txt"))
}
