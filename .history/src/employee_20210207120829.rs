use crate::ided::Ided;
use crate::new::New;
extern crate name;

pub struct Employee {
    name: String,
    tax_no: u64,
}

impl Ided for Employee {
    fn ided(&self) -> u64 {
        self.tax_no
    }
}

impl Employee {
    fn new(name: String) -> Self {
        Employee { name, tax_no }
    }
}
