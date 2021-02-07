use trait_test::{employee::Employee, ided::Ided, professor::{DoctorTitles, Professor}};

fn main() {
    let bunch_of_impls = vec![
        Employee::new(String::from("Josef")),
        Employee::new(String::from("Wusel")),
        Professor::new([DoctorTitles::])
    ];

    for i in bunch_of_impls {
        i.ided();
    }
}
