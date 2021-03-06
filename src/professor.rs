use crate::ided::Ided;

pub enum DoctorTitles {
    Phil,
    Med,
    Jur,
}

pub struct Professor {
    doctor_titles: Vec<DoctorTitles>,
    service_no: u64,
}

impl Ided for Professor {
    fn ided(&self) -> u64 {
        self.service_no
    }
}

impl Professor {
    pub fn new(doctor_titles: Vec<DoctorTitles>, service_no: u64) -> Professor {
        Professor {
            doctor_titles,
            service_no,
        }
    }
}
