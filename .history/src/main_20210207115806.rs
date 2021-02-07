
fn main() {
}

fn concurrent() -> JoinHandle<()> {
    let x = Box::new(222);
    let s = thread::spawn(move || println!("x : {}", *x));
    return s;
}
