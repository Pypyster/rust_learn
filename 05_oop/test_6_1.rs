mod test_6;

use std::cell::RefCell;
use std::rc::Rc;

use test_6::{ChatApp, Room, User};

fn main() {
    let mut app = ChatApp::new("Mega Chat".to_string());

    let alice = Rc::new(RefCell::new(User::new("Alice".to_string())));
    let bob = Rc::new(RefCell::new(User::new("Bob".to_string())));
    let charlie = Rc::new(RefCell::new(User::new("Charlie".to_string())));
    let diana = Rc::new(RefCell::new(User::new("Diana".to_string())));
    let eve = Rc::new(RefCell::new(User::new("Eve".to_string())));
    let frank = Rc::new(RefCell::new(User::new("Frank".to_string())));

    let general = Rc::new(RefCell::new(Room::new("general".to_string())));
    let rust_room = Rc::new(RefCell::new(Room::new("rust".to_string())));
    let games = Rc::new(RefCell::new(Room::new("games".to_string())));
    let music = Rc::new(RefCell::new(Room::new("music".to_string())));

    app.add_user(Rc::clone(&alice));
    app.add_user(Rc::clone(&bob));
    app.add_user(Rc::clone(&charlie));
    app.add_user(Rc::clone(&diana));
    app.add_user(Rc::clone(&eve));
    app.add_user(Rc::clone(&frank));

    app.add_room(Rc::clone(&general));
    app.add_room(Rc::clone(&rust_room));
    app.add_room(Rc::clone(&games));
    app.add_room(Rc::clone(&music));

    general.borrow_mut().add_user(Rc::clone(&alice));
    general.borrow_mut().add_user(Rc::clone(&bob));
    general.borrow_mut().add_user(Rc::clone(&charlie));
    general.borrow_mut().add_user(Rc::clone(&diana));

    rust_room.borrow_mut().add_user(Rc::clone(&alice));
    rust_room.borrow_mut().add_user(Rc::clone(&charlie));
    rust_room.borrow_mut().add_user(Rc::clone(&eve));

    games.borrow_mut().add_user(Rc::clone(&bob));
    games.borrow_mut().add_user(Rc::clone(&diana));
    games.borrow_mut().add_user(Rc::clone(&frank));

    music.borrow_mut().add_user(Rc::clone(&alice));
    music.borrow_mut().add_user(Rc::clone(&eve));
    music.borrow_mut().add_user(Rc::clone(&frank));

    println!("=== APP INFO ===");
    println!("App: {}", app.name());
    println!("Users count: {}", app.all_users().len());
    println!("Rooms count: {}", app.all_rooms().len());
    println!();

    println!("=== USER CHATS AFTER JOIN ===");
    println!("Alice chats: {:?}", alice.borrow().chats());
    println!("Bob chats: {:?}", bob.borrow().chats());
    println!("Charlie chats: {:?}", charlie.borrow().chats());
    println!("Diana chats: {:?}", diana.borrow().chats());
    println!("Eve chats: {:?}", eve.borrow().chats());
    println!("Frank chats: {:?}", frank.borrow().chats());
    println!();

    general
        .borrow_mut()
        .post_message(Rc::clone(&alice), "Всем привет!".to_string());
    general
        .borrow_mut()
        .post_message(Rc::clone(&bob), "Привет, Alice!".to_string());
    general
        .borrow_mut()
        .post_message(Rc::clone(&charlie), "Добрый вечер всем".to_string());
    general
        .borrow_mut()
        .post_message(Rc::clone(&diana), "Как дела?".to_string());

    rust_room
        .borrow_mut()
        .post_message(Rc::clone(&alice), "Кто пишет на Rust каждый день?".to_string());
    rust_room
        .borrow_mut()
        .post_message(Rc::clone(&charlie), "Я сейчас изучаю ownership".to_string());
    rust_room
        .borrow_mut()
        .post_message(Rc::clone(&eve), "А я дошла до Rc<RefCell<T>>".to_string());

    games
        .borrow_mut()
        .post_message(Rc::clone(&bob), "Кто играет вечером?".to_string());
    games
        .borrow_mut()
        .post_message(Rc::clone(&diana), "Я в деле!".to_string());
    games
        .borrow_mut()
        .post_message(Rc::clone(&frank), "Во что идём?".to_string());

    music
        .borrow_mut()
        .post_message(Rc::clone(&alice), "Кто что слушает?".to_string());
    music
        .borrow_mut()
        .post_message(Rc::clone(&eve), "Сейчас много lo-fi".to_string());
    music
        .borrow_mut()
        .post_message(Rc::clone(&frank), "У меня сегодня джаз".to_string());

    println!("=== MESSAGE COUNTS ===");
    println!("general: {}", general.borrow().messages().len());
    println!("rust: {}", rust_room.borrow().messages().len());
    println!("games: {}", games.borrow().messages().len());
    println!("music: {}", music.borrow().messages().len());
    println!();

    println!("=== GENERAL MESSAGES ===");
    for msg in general.borrow().messages() {
        let msg_ref = msg.borrow();
        let author = msg_ref.author();
        println!(
            "[{}] {}: {}",
            msg_ref.time(),
            author.borrow().name(),
            msg_ref.text()
        );
    }
    println!();

    println!("=== RUST ROOM MESSAGES ===");
    for msg in rust_room.borrow().messages() {
        let msg_ref = msg.borrow();
        let author = msg_ref.author();
        println!(
            "[{}] {}: {}",
            msg_ref.time(),
            author.borrow().name(),
            msg_ref.text()
        );
    }
    println!();

    println!("=== USER LEAVES ROOM ===");
    general.borrow_mut().remove_user(&alice);
    println!("Alice chats after leaving general: {:?}", alice.borrow().chats());
    println!("Alice online: {}", alice.borrow().is_online());
    println!("general users left: {}", general.borrow().users().len());
    println!();

    println!("=== REMOVE USER FROM APP ===");
    app.remove_user(&bob);
    println!("Bob chats after removing from app: {:?}", bob.borrow().chats());
    println!("Bob online after removing from app: {}", bob.borrow().is_online());
    println!("Users left in app: {}", app.all_users().len());
    println!();

    println!("=== FINAL STATE ===");
    println!("Alice chats: {:?}", alice.borrow().chats());
    println!("Charlie chats: {:?}", charlie.borrow().chats());
    println!("Diana chats: {:?}", diana.borrow().chats());
    println!("Eve chats: {:?}", eve.borrow().chats());
    println!("Frank chats: {:?}", frank.borrow().chats());
}
