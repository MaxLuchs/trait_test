use trait_test::{
    employee::Employee,
    ided::{use_ided_dyn, Ided},
    professor::{DoctorTitles, Professor},
    student::Student,
};
use trait_test::ided::{use_ided_static, Personality};

fn main() {
    // dynamically typed with Boxed dyns:
    let bunch_of_dyns: Vec<Box<dyn Ided>> = vec![
        Box::new(Employee::new(String::from("Josef"))),
        Box::new(Employee::new(String::from("Wusel"))),
        Box::new(Professor::new(vec![DoctorTitles::Med], 123_u64)),
    ];

    for i in bunch_of_dyns {
        use_ided_dyn(i);
    }

    // statically typed with Enum of subtypes:
    let bunch_of_static: Vec<Personality> = vec![
        Personality::Employee(Employee::new(String::from("Josef"))),
        Personality::Employee(Employee::new(String::from("Wusel"))),
        Personality::Professor(Professor::new(vec![DoctorTitles::Med], 123_u64))
    ];

    for i in bunch_of_static {
        use_ided_static(&i);
    }
}
