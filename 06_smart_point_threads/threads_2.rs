use futures::executor::block_on;
use std::{f32::consts::E, thread::{self, JoinHandle}, time::Duration};

use thread_help::*;
fn main() {
    println!("This is main thread: {:?}", thread::current().id());

    let play = play_in_thread();
    let eat = eat_in_thread();
    play.join().unwrap();
    eat.join().unwrap();

    std::thread::spawn(move ||{
        println!("Print from another thread!");
    });

    block_on(task_print());

    println!("______________________");
    block_on(play_eat());

    println!("______________________");
    block_on(paly_eat_concur());
}

