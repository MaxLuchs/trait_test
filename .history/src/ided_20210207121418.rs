pub trait Ided {
    fn ided(&self) -> u64;
}

pub fn use_ided_(item: impl Ided) {
    println!("use ided: {}", item.ided())
}
