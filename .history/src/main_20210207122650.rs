use trait_test::{employee::Employee, ided::{Ided, use_ided_dyn}, professor::{DoctorTitles, Professor}};

fn main() {
    let bunch_of_impls: Vec<Box<dyn Ided>> = vec![
        Box::new(Employee::new(String::from("Josef"))),
        Box::new(Employee::new(String::from("Wusel"))),
        Box::new(Professor::new(vec![DoctorTitles::Med], 123_u64)),
    ];

    for i in bunch_of_impls {
        use_ided_dyn(i).ided();
    }
}
