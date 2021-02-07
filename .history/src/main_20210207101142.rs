use std::{fs::File, io::Read, path::Path, thread};

fn main() {
    println!("Hello, world!");
    let mut file = File::open(&Path::new("test.txt")).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("*** file content : {}", content);
}

fn concurrent() {
    let x = Box::new(222);
    let s = thread::spawn(|| println!("x : {}", *x));
}
