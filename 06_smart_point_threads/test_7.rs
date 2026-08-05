#![warn(clippy::all, clippy::pedantic)]
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone,Debug)]
struct  Dog {
    name: String,
    age: u8,
    parent: Option<Box<Dog>>,
}

#[derive(Debug)]
struct  Cat {
    name: String,
    age: u8,
    parent: Option<Rc<Cat>>,
}

#[derive(Debug)] 
struct Hamster{
    color: String,
    age: u8,
    parent: Option<Rc<RefCell<Hamster>>>,
}


fn main() {
    let a = 5;
    let b = &a;
    println!("{} address: {:p}",a, b);

    let a = vec![-3;8];// владеет данными и явл умным указателем 
    let b = &a; // обычный указатель
    let c = (*b).clone();

    let s1 = String::from("Merphy");
    let b = s1.deref();
    let bor = &s1;
    let my_str = &(*bor)[..];

    println!("________________________");
    let a = Box::new(42);
    let b = &a;
    let c = a.deref();
    let d = *a;
    let e = &d;
    println!("{}", c==e);

    let mut dog1 = Box::new(Dog{
        name: s1,
        age: 8,
        parent: None,
    });

    let dog2 = Dog{
        name: "Reks".to_string(),
        age: 2,
        parent: Some(dog1.clone()),
    };

    dog1.age = 12;
    println!("{:?}. Changed age {}", dog2.parent, dog1.age);
    
    println!("________________________");

    let cat1 = Rc::new(Cat{
        name: "Silvia".to_string(),
        age: 18,
        parent: None,
    });

    let cat2 = Rc::new(Cat{
        name: "Reks".to_string(),
        age: 6,
        parent: Some(Rc::clone(&cat1)),
    });
    let cat3 = Cat{
        name:"Dodo".to_string(),
        age: 2,
        parent: Some(Rc::clone(&cat2)),
    };

    println!("Original parent: {:?}", cat3.parent);
    println!("Elder cat ptr: {}. Middle cat ptr: {}", Rc::strong_count(&cat1), Rc::strong_count(&cat2));

    println!("________________________");

    let ham1 = Rc::new(RefCell::new(Hamster{
        color: "Black".to_string(),
        age: 10,
        parent:None,
    }));

    let ham2 = Rc::new(RefCell::new(Hamster{
        color: "Brown".to_string(),
        age: 7,
        parent: Some(Rc::clone(&ham1)),
    }));
    
    let ham3 = Rc::new(RefCell::new(Hamster{
        color: "Grey".to_string(),
        age: 3,
        parent:Some(Rc::clone(&ham2)),
    }));

    println!("Original:");
    par_iter(&ham3);
    ham3.borrow().parent.as_ref().unwrap().borrow_mut().color = "Pink".to_string();
    println!("Changed:");
    par_iter(&ham3);


}

fn par_iter(child: &Rc<RefCell<Hamster>>) {
    let mut current = Some(Rc::clone(child));

    while let Some(curent_ham) = current.take(){
        let ham_ref = curent_ham.borrow();
        println!("Get hamster color: {} {} years old", ham_ref.color, ham_ref.age);

        if let Some(parent) = ham_ref.parent.as_ref(){
            let per_ref = parent.borrow();
            println!("Parent info: color {}, age {}", per_ref.color, per_ref.age);
            current = Some(Rc::clone(parent));
        }
        println!("++++++++++");
    }

}