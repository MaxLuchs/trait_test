use crate::ided::Ided;

pub struct Professor {
    doctor_titles: Vec<String>,
    service_no: u64,
}

impl Ided for Professor {
    fn ided(&self) -> u64 {
        self.service_no
    }
}


impl Student {
    fn new(university_id: u64) -> Student {
        Student { university_id }
    }
}
