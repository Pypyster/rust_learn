use::std::rc::Rc;
struct Dev {
    lang: String,
    ex: u32
}

struct Project {
    name: String,
    developers: Rc<Dev>
}

fn main(){
    let dev = Dev{
        lang: "Rust".to_string(),
        ex: 5
    };

    let dev_in_box = Rc::new(dev);

    {
        let google_maps = Project{
        name: "Google map".to_string(),
        developers: Rc::clone(&dev_in_box)
        };
        println!("Ref count: {}", Rc::strong_count(&dev_in_box));
    }
    

    let google_pay = Project{
        name: "Google pay".to_string(),
        developers: Rc::clone(&dev_in_box)
    };

    println!("Ref count: {}", Rc::strong_count(&dev_in_box));

    let x = Rc::new(7);
    assert_eq!(Rc::try_unwrap(x), Ok(7));

}