/*
основы ооп:
инкапсуляция (private) поля
наследование 
полиморфизм
*/
mod some_mod{
    use chrono::Utc;
    pub struct Project{
        pub members: Vec<Box<dyn TeamMember>>
    }

    pub trait TeamMember{
        fn do_task(&self) {
            println!("Do task: {}", Utc::now());
        }
    }
    
    pub struct Dev {
        lang: String,
        exp: u8,
        tasks: Vec<String>
    }
    pub struct QA{

    }

    impl TeamMember for QA {
        
    }

    impl TeamMember for Dev{
        fn do_task(&self) {
            println!("Do dev task: {}", Utc::now());
        }
    }

    impl Dev {
        pub fn new(lang: String)-> Dev{
            Dev { lang, exp: 0, tasks: Vec::new()}
        }
        pub fn change_lang ( &mut self, new_lang: String) {
            if new_lang == "JS".to_string() {
                self.lang = new_lang;
            }
        }
        fn list_task (&self) -> &[String] {
            &self.tasks
        }
    }
}

fn main() {
    let mut  dev = some_mod::Dev::new("Rust".to_string());
    dev.change_lang("php".to_string());

    let qa = some_mod::QA{};

    let mut project  = some_mod::Project{members: Vec::new()};
    project.members.push(Box::new(dev));
    project.members.push(Box::new(qa));

    for i in project.members.iter(){
        i.do_task();
    }
}


