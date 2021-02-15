use crate::student::Student;
use crate::professor::Professor;
use crate::employee::Employee;

pub trait Ided {
    fn ided(&self) -> u64;
}

// dynamic Trait usage:
pub fn use_ided_impl(item: impl Ided) {
    println!("use ided: {}", item.ided())
}

pub fn use_ided_dyn(item: Box<dyn Ided>) {
    println!("use ided: {}", item.ided())
}

pub fn use_ided_static(item: &Personality) {
    println!("use ided: {}", item.ided())
}

pub fn use_ided_generic<T: Ided>(item: T) {
    println!("use ided: {}", item.ided())
}


// static Trait usage:

pub enum Personality {
    Employee(Employee),
    Professor(Professor),
    Student(Student),
}

impl Ided for Personality {
    fn ided(&self) -> u64 {
        match self {
            Personality::Employee(e) => e.ided(),
            Personality::Professor(p) => p.ided(),
            Personality::Student(s) => s.ided(),
        }
    }
}
