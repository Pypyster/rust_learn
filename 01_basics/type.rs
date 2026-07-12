fn main() {
    let num = 50;
    println!("Results: {}", num);

    let num = num + 10;
    println!("Results: {}", num);

    let num: u8 = 50;
    println!("Results: {}", num);

    let num: i16 = -4500;
    let res = 2.546 + num as f32;
    println!("Results: {}", res);

    let num: u64 = 10000000;
    println!("Results: {}", num);

    // float
    let num: f32 = 5.443414;
    println!("Results: {}", num);

    // bool
    let is_has_car: bool = true;
    println!("Results: {}", is_has_car);
    
    // char
    let sym: char = '3';
    println!("Results: {}", sym);

    const USER_MAX_SCORE: u32 = 1_000_000;
    println!("Results: {}", USER_MAX_SCORE);
 
}