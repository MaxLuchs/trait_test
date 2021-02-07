use trait_test::{
    employee::Employee,
    ided::{use_ided_dyn, use_ided_impl, Ided},
    professor::{DoctorTitles, Professor},
    student::Student,
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
    for i in vec![
        Box::new(gen_ideds(0)),
        Box::new(gen_ideds(1)),
        Box::new(gen_ideds_2(0)),
        Box::new(gen_ideds_2(1)),
    ] {
        use_ided_impl(i);
    }
}