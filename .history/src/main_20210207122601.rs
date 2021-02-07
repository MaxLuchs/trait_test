use trait_test::{
    employee::Employee,
    ided::Ided,
    professor::{DoctorTitles, Professor},
};

fn main() {
    let bunch_of_impls: Vec<Box<dyn Ided>> = vec![
        Box::new(Employee::new(String::from("Josef")),
        Box::new(Employee::new(String::from("Wusel")),
        Professor::new(vec![DoctorTitles::Med], 123_u64),
    ];

    for i in bunch_of_impls {
        i.ided();
    }
}
