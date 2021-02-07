use crate::ided::Ided;
pub struct Student {
    university_id: u64,
}


impl Ided for Professor {
    fn ided(&self) -> u64 {
        self.service_no
    }
}