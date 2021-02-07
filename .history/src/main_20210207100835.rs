use std::{fs::File, io::Read, path::Path};




fn main() {
    println!("Hello, world!");
    let mut file = File::open(&Path::new("test.txt")).unwrap();
    let mut content = String::new();
    file.rearead_to_string(&mut content).unwrap();
    println!("*** file content : {}", content);
}
