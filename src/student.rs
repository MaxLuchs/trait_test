use crate::ided::Ided;
extern crate rand;
pub struct Student {
    university_id: u64,
}

impl Ided for Student {
    fn ided(&self) -> u64 {
        self.university_id
    }
}

impl Student {
    pub fn new() -> Student {
        Student {
            university_id: rand::random::<u64>(),
        }
    }
}
