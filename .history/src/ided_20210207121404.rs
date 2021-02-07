pub trait Ided {
    fn ided(&self) -> u64;
}

pub fn use_ided(item: Ided) {
    println!("use ided: {}", ided())
}
