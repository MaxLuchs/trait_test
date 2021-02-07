use crate::ided::Ided;
pub struct Student {
    university_id: u64,
}

impl Ided for Student {
    fn ided(&self) -> u64 {
        self.university_id
    }
}

impl Student {
    new
}
