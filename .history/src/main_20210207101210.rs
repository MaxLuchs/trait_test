use std::{fs::File, io::Read, path::Path, thread};

fn main() {

}

fn concurrent() {
    let x = Box::new(222);
    let s = thread::spawn(|| println!("x : {}", *x));
}
