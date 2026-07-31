//RefCell шаблон внутренней изменяемости, изменяет данные даже если ссылки на данные неизменяемые
use core::cell::RefCell;
struct Developer{
    lang: String,
    tasks: RefCell<Vec<String>>
}

trait Assignmet {
    fn assign(&self, task_name: String);
}

impl Assignmet for Developer {
    fn assign(&self, task_name: String) {
        if !task_name.contains("hack") {
            &self.tasks.borrow_mut().push(task_name);
        }
    }
}

fn main() {
    let dev = Developer{
        lang: "Rust".to_string(),
        tasks: RefCell::new(vec![])
    };
    dev.assign("Create facebook".to_string());
    dev.assign("Delete prev correction".to_string());

    let mut borr_mut1 = dev.tasks.borrow_mut();
    let mut borr_mut2 = dev.tasks.borrow_mut();
    /*borr_mut1.push("Hack anyone".to_string());
    borr_mut2.push("Create new account".to_string());
    вызовет panic!
    */
}