
fn main(){
    let res = divi(10, 0);
    match res {
        Ok(value) => println!("{value}"),
        Err(e) => println!("Error in function: {e}"),
    }
    let ruslt = divi(10, 1).unwrap(); // err вызывает ощибку panicked и стопает всю программу

    let  optin = find_el(vec![1, 2, 6, 7, 8, 9, 10, 12], 3); // unwrap
    match optin {
        Some(value) => println!("EL is finded {value}"),
        None => println!("El didnt find!"),
    }
}

fn divi (a: i32, b: i32) -> Result<i32, String>{
    if b == 0{
       Err(String::from("Division by zero!"))  // перечисление Result (ok, err)
    } else{
        Ok(a / b)
    }
}

fn find_el(v: Vec<i32>, value: i32) -> Option<usize>{
    v.iter().position(|&x| x == value)  // замыкание 
}