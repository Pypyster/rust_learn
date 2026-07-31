use std::{borrow::Cow, iter, str::Matches};

fn to_lowwercase_need(text: &str) -> Cow<str> {
    if !text.chars().any(char::is_uppercase) {
        Cow::Borrowed(text)
    } else {
        Cow::Owned(text.to_lowercase())
    }
}

#[derive(Clone,Debug)]
struct  Employee {
    name: String,
    salary: u32,
    bonuse: u16,
}

fn apply_bonus (employee: &mut Cow<Employee>) {
    if employee.bonuse == 0 {
        employee.to_mut().bonuse = employee.salary as u16 / 10;
    }
}
#[test] 
fn test_to_lowercase_need() {
    assert!(matches!(
        to_lowwercase_need("hello!"),
        Cow::Borrowed(_)
    ));
    assert!(matches!(
        to_lowwercase_need("hEllO"),
        Cow::Owned(_)
    ) );
}

fn main() {
    let emploer: Vec<Employee> = vec![
        Employee{
            name: "Ferris".to_string(),
            salary: 10000,
            bonuse: 0
        },
        Employee{
            name: "Guffy".to_string(),
            salary: 900,
            bonuse: 200
        },
        Employee{
            name: "Merphy".to_string(),
            salary: 56900,
            bonuse: 100
        },
    ];
    let mut emploer_bonus: Vec<Cow<Employee>> = emploer.iter().map(Cow::Borrowed).collect();

    for empl in emploer_bonus.iter_mut() {
        apply_bonus(empl);
        println!("{:?}", empl);
    }
}