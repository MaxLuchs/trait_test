use trait_test::{
    employee::Employee,
    ided::{use_ided_dyn, Ided},
    professor::{DoctorTitles, Professor},
};

fn main() {
    // dynamic
    let bunch_of_dyns: Vec<Box<dyn Ided>> = vec![
        Box::new(Employee::new(String::from("Josef"))),
        Box::new(Employee::new(String::from("Wusel"))),
        Box::new(Professor::new(vec![DoctorTitles::Med], 123_u64)),
    ];

    for i in bunch_of_dyns {
        use_ided_dyn(i);
    }

    // static

    let bunch_of_static: Vec<Ided> = vec![
        Employee::new(String::from("Josef")),
        Employee::new(String::from("Wusel")),
        Professor::new(vec![DoctorTitles::Med], 123_u64),
    ];

    for i in bunch_of_static {
        use_ided_dyn(i);
    }
}
