use trait_test::{
    employee::Employee,
    ided::{use_ided_dyn, use_ided_impl, Ided},
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
    print_ideds(vec![
        Employee::new(String::from("Josef")) as impl Ided,
        Employee::new(String::from("Wusel")) as impl Ided,
        Professor::new(vec![DoctorTitles::Med], 123_u64) as impl Ided,
    ]);
    for i in ideds {
        use_ided_impl(i);
    }
}

fn gen_ideds() -> Vec<impl Ided> {
    
}
