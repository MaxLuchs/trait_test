use std::{fs::File, io::Read, path::Path, thread};

fn main() {
    concurrent();
}

fn concurrent() -> Join{
    let x = Box::new(222);
    let s = thread::spawn(|| println!("x : {}", *x));
    return s;
}
