
#[allow(unused_variables)]

fn main() {
    let get_answer = |x,y| x+y;
    println!("{:?}", get_answer(10,20));

    println!("{:?}",(|x,y| x*y)(4 as f32, 3.8));

    println!("______________________-");

    fn add(x:i16, y: i16) -> i16 {x + y}

    fn calculate_print (x: i16, y:i16, calc: fn(i16, i16)-> i16){
        let res = calc(x,y);
        println!("{:?}", res);
    }

    calculate_print(4, -87, add);
    calculate_print(4, 7, |x, y| x / y);
    println!("______________________-");

    fn calc_my_print(x:i16, y: i16, calc: Box<dyn MyAdd>){
        let res = calc.add(x, y);
        println!("{:?}", res);
    }

    let z = -8;
    let closure = MyClosure{z};
    calc_my_print(154, 0, Box::new(closure));
    println!("______________________-");

    let z = -3;
    let ff: Box<dyn Fn(i16, i16)-> i16> = Box::new(|x:i16, y:i16|{
        x + y + z
    });

    fn calc_and_print<'a> (x:i16, y: i16, calc: Box<dyn Fn(i16, i16)-> i16 +'a>){
        let res = calc(x,y);
        println!("{:?}", res);
    }
    calc_and_print(1, 7, ff);

    println!("______________________-");
    
    let mut  res: i32 = 0;
    let calc = |x:i32, y:i32|{
        res = x + y;
        println!("{:?}", res);
    };

    let mut fn_mut: Box<dyn FnMut(i32, i32)> = Box::new(|x:i32, y:i32|{
        res = x+y
    });
    fn_mut(1, 3);

    drop(fn_mut);
    println!("{:?}", res);

    println!("______________________-");
    let s  = String::from("Hello");
    let f = move||println!("{:?}", s);
    f();
    let s = "Bye bye";
    let f_once: Box<dyn FnOnce()> = Box::new(move|| println!("{:?}", s));
    f_once();
    


}
struct MyClosure{
    z: i16,
}

trait MyAdd {
    fn add(&self, x: i16, y:i16) -> i16;
}

impl MyAdd for MyClosure  {
    fn add(&self, x: i16, y:i16) -> i16 {
        x + y + &self.z
    }
}