use std::ops::{Deref,DerefMut};


unsafe fn dangerouse(){
    println!("Dangerous operation!");
}

unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

struct  MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut<Self as std::ops::Deref> ::Target{
        &mut self.0
    }
}
fn main(){

    let x = 10;
    let p = &x as *const i32;
    unsafe {
        println!("{:p}: {}",p, *p);
        dangerouse();
        println!("{}", abs(-87));
    }
    let x = 5.0;
    let y = &x;
    assert_eq!(5.0,x);
    assert_eq!(*y,5.0);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5,x);
    assert_eq!(*y,5);

    let mut m = MyBox::new(String::from("Merphy"));
    let mut_ref = &mut (*m);

    hello(&m);// неявное разыменованное приведение let der: &str = String::from("Rust").deref();


}

fn hello (name: &str) {
    print!("Hello, {}", name);
}
