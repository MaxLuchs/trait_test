use std::string;

use trait_test::employee::Employee;

fn main() {
    let bunch_of_impls: Vec<impl Ided> = vec![
        Employee::new(string.from("Josef")()),
        Employee::new("Wusel"),


    ]
}