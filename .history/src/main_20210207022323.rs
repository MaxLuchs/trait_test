use std::{fs::File, io::Read, path::Path};

fn main() {
    println!("Hello, world!");
    let file = File::open(&Path::new("test.txt")).unwrap();
    let content
    file.read(buf)
}
