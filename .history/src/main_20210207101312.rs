use std::{fs::File, io::Read, path::Path, thread};

use thread::JoinHandle;

fn main() {
    let handle = concurrent();
    handle.join()
}

fn concurrent() -> JoinHandle<()> {
    let x = Box::new(222);
    let s = thread::spawn(|| println!("x : {}", *x));
    return s;
}
