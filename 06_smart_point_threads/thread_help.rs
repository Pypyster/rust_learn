use std::{thread::JoinHandle, time::Duration};
use std::thread;

pub async fn task_print() {
    println!("Print from future");
}

pub async fn eat() {
        let current_id = thread::current().id();
        for i in 0..6{
            println!("task eat -> thread {:?}", current_id);
            async_std::task::sleep(Duration::from_millis(500)).await;
            println!("task eat -> thread {:?}", current_id);
        };
}

pub async fn play() {
        let current_id = thread::current().id();
        for i in 0..6{
            println!("task play -> thread {:?}", current_id);
            async_std::task::sleep(Duration::from_millis(500)).await;
            println!("task play -> thread {:?}", current_id);
        };
}

pub async fn play_eat() {
    println!("Eat and play one by one");
    eat().await;
    play().await;
}

pub async fn paly_eat_concur() {
    println!("Play and eat at same time");
    let f1 = eat();
    let f2 = play();
    futures::join!(f1,f2);
}

pub async fn bad_sleep(){
    thread::sleep(Duration::from_millis(800));// не может быть использован пока не проснется
    println!("Block  thread (");
}

pub async fn good_sleep() {
    async_std::task::sleep(Duration::from_millis(800)).await;// поток можно переиспользовать
    println!("Good sleep")
}
pub fn eat_in_thread() -> JoinHandle<()> {
    std::thread::spawn(||{
        let current_id = thread::current().id();
        for i in 0..6{
            println!("eat -> thread {:?}", current_id);
            std::thread::sleep(Duration::from_secs(2));
            println!("eat -> thread {:?}", current_id);
        }
    })
}

pub fn play_in_thread() -> JoinHandle<()> {
    std::thread::spawn(||{
        let current_id = thread::current().id();
        for i in 0..4{
            println!("play -> thread {:?}", current_id);
            std::thread::sleep(Duration::from_secs(2));
            println!("play -> thread {:?}", current_id);
        }
    })
}

pub fn play_game() -> u8{
    36
}