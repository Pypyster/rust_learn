mod math;
macro_rules! my_print {
    ($msg:expr) => {   //msg: expr это паттерн msg переменная expr ожидаем выражение  
        println!("{}", $msg);
    };
}


fn main(){
    test();
    add(15,24);

    let mut user = String:: from("Pypypy");
    great_user(& mut user);
    
    let res1 = add_new(19, 7);
    let res2 = add_new(5,-1);
    println!("Res1: {}", res1);
    println!("Res2: {}", res2);

    let nums = (5,14);
    println!("Mult is {}", mult(&nums));

    my_print!("HeHeHe");

    let sum = math :: add(5, -9);
    my_print!(sum);

    let minus = math :: minus(5, -9);
    my_print!(minus);
}

fn test(){
    println!("Hello");
}

fn add (a: i16, b: i16){
    let sum = a + b;
    println!("Result: {}", sum);
}

fn great_user (name : &mut String){
    *name = String :: from("Kaka");
    println!("Hi {}", name);
}

fn add_new(a: i16, b: i16) -> i16{
    let res = a +b;
    res
}

fn mult (data: &(i32, i32)) -> i32{
    data.0 * data.1
}