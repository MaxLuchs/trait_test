use trait_test::employee::Employee;

fn main() {
    let bunch_of_impls: Vec<impl Ided> = vec![
        Employee::new(String.from("Josef").into::<String>()),
        Employee::new("Wusel"),


    ]
}
