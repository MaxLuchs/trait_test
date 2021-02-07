use crate::ided::Ided;
use crate::new::New;
pub struct Employee {
    name: String,
    tax_no: u64,
}

impl Ided for Employee {
    fn ided(&self) -> u64 {
        self.tax_no
    }
}

impl New for Employee {
    fn new() -> Self {
        
    }
}