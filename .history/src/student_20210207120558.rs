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
    fn new(university_id: u64) -> Student {
        Student { university_id }
    }
}

impl Student {
    pub fn new(university_id: u64) -> Student {
        Student {
            
        }
    }
}