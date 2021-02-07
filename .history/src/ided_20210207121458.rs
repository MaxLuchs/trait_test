pub trait Ided {
    fn ided(&self) -> u64;
}

pub fn use_ided_impl(item: impl Ided) {
    println!("use ided: {}", item.ided())
}

pub fn use_ided_dyn(item: Box<dyn Ided>) {
    println!("use ided: {}", item.ided())
}
