use crate::ided::Ided;

pub struct Professor {
    doctor_titles: Vec<String>,
    service_no: u64,
}

impl Ided for Employee {
    fn ided(&self) -> u64 {
        self.tax_no
    }
}