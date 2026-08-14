use std::io;
fn main(){
    //  memory
    let mut user_data = String :: new();
    io::stdin().read_line(&mut user_data).expect("Fail ti read info");
    println!("Result: {}", user_data);

    let mut num1= String :: new();
    let mut num2 = String ::new();
    
    println!("Enter num1:");
    io::stdin().read_line(&mut num1).expect("Fail ti read info");
    
    println!("Enter num2:");
    io::stdin().read_line(&mut num2).expect("Fail ti read info");
    
    let data1 : i16 = num1.trim().parse().expect("Please enter a valid number ");
    let data2 : u8 = num2.trim().parse().expect("Please enter a valid number ");
    

    println!("Result 1: {}, Result 2: {}", data1, data2);

    let  res: i16 = data1 + data2 as i16;
        println!("Result add: {}", res);

}