use crate::ided::Ided;

pub enum

pub struct Professor {
    doctor_titles: Vec<String>,
    service_no: u64,
}

impl Ided for Professor {
    fn ided(&self) -> u64 {
        self.service_no
    }
}

impl Professor {
    fn new(doctor_titles: Vec<String>, service_no: u64) -> Professor {
        Professor {
            doctor_titles,
            service_no,
        }
    }
}
