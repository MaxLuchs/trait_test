use std::{fs::File, io::Read, path::Path};

fn main() {
    println!("Hello, world!");
    let mut file = File::open(&Path::new("test.txt")).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    console.log("*** file content : {")
}