fn main(){
    let s1 = String:: from("Hello");
    let len = calculate_length(&s1); // заимстование по неизменяемой ссылке
 
    println!("Length of {} is {}", s1, len);

    let mut s = String :: from("Lalaland");
    change(&mut s); // изменяемое заимствование

    println!("{}", s);

    let r1 = &s; // неизменяемая ссылка
    let r2 = &s;
    //let mut r3 = &mut s; //ошибка т.к. существ неизмен ссылки

    println!("{:p}, {}", r1, r2 );
}

fn calculate_length (s: &String) -> usize {
    s.len()    
}

fn change(s: &mut String){
    s.push_str(" is a film");
}