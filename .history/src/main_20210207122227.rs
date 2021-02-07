use trait_test::{employee::Employee, ided::Ided};

fn main() {
    let bunch_of_impls = vec![
        Employee::new(String::from("Josef")),
        Employee::new(String::from("Wusel")),
    ];

    for i in bunch_of_impls {
        i.ided();
    }
}
