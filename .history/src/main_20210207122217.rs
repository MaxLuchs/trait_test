use trait_test::employee::Employee;

fn main() {
    let bunch_of_impls = vec![
        Employee::new(String::from("Josef")),
        Employee::new(String.from("Wusel")),
    ];

    for i in bunch_of_impls {
        i.into();
    }
}