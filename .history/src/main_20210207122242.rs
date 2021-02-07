use trait_test::{employee::Employee, ided::Ided, professor::Professor};

fn main() {
    let bunch_of_impls = vec![
        Employee::new(String::from("Josef")),
        Employee::new(String::from("Wusel")),
        Professor
    ];

    for i in bunch_of_impls {
        i.ided();
    }
}
