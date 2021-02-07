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
    for i in vec![gen_ideds(0), gen_ideds(1), gen_ideds_2(0), Box<gen_ideds_2(1)] {
        use_ided_impl(i);
    }
}

fn gen_ideds(i: u32) -> impl Ided {
    match i {
        0 => Professor::new(vec![DoctorTitles::Med], 123_u64),
        _ => Professor::new(vec![DoctorTitles::Phil], 333_u64),
    }
}

fn gen_ideds_2(i: u32) -> impl Ided {
    match i {
        0 => Student::new(),
        _ => Student::new(),
    }
}
