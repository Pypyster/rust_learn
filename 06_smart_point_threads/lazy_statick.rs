#[macro_use]
extern crate lazy_static;
use std::{os::windows::thread, sync::Mutex};

lazy_static! {
    static ref PLAYBOY: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn save(version: &str) {
    let mut db = PLAYBOY.lock().map_err(|_| "Failed to acquire MutexGuard!").unwrap();
    db.push(version.to_string());
}
fn main() {
    save("1953 Мэрлин Монро");
    save("1955 Заячий загар");
    save("1967 Отражение в кроличьих очках");
    save("1971 Первая темнокожая");
    let handle = std::thread::spawn(||{
        save("1985 Мадонна");
    });
    save("1990 Дональд Трамп");
    save("1993 Анна Николь-Смит");
    save("2009 Мардж Симпсон");
    save("2014 Кейт Мосс");
    save("2020 Ивлеева");

    {
        let db = PLAYBOY.lock()
        .map_err(|_| "Main thread failed to acquire")
        .unwrap();

        db.iter().enumerate().for_each(|(i,item)|
        println!("{}: {}", i, item));
    }
}