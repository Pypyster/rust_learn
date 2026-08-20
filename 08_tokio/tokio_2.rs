use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{oneshot, watch};
use tokio::{fs, process, signal};
use tokio::time::{self, sleep};

async fn sleepy() {
    log::info!("Start sleeping");
    time::sleep(time::Duration::from_secs_f32(10.41)).await;
    log::info!("Ending sleepy");
}

async fn demo_oneshot() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send("Problem was solved").unwrap();
    });

    match rx.await {
        Ok(msg) => println!("You send: {msg}"),
        Err(_) => println!("Not found sender"),
    }
}

async fn demo_watch_multiple_receivers() {
    let (tx, mut rx) = watch::channel("Something work");

    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        tx.send("Busy").unwrap();

        sleep(Duration::from_secs(2)).await;
        tx.send("Ups, it brokes").unwrap();
    });

    let mut rx1 = rx.clone();
    let mut rx2 = rx.clone();
    let mut rx3 = rx;

    let h1 = tokio::spawn(async move {
        while rx1.changed().await.is_ok() {
            println!("[Receiver 1] Status: {}", *rx1.borrow());
        }
        println!("[Receiver 1] channel closed");
    });

    let h2 = tokio::spawn(async move {
        while rx2.changed().await.is_ok() {
            println!("[Receiver 2] Status: {}", *rx2.borrow());
        }
        println!("[Receiver 2] channel closed");
    });

    let h3 = tokio::spawn(async move {
        while rx3.changed().await.is_ok() {
            println!("[Receiver 3] Status: {}", *rx3.borrow());
        }
        println!("[Receiver 3] channel closed");
    });

    let _ = tokio::join!(h1, h2, h3);
}

async fn demo_watch_send_replace() {
    let (tx, mut rx) = watch::channel("Something work");

    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        let previous = tx.send_replace("Busy");
        println!("It was: {}", previous);

        sleep(Duration::from_secs(2)).await;
        let previous = tx.send_replace("Ups, it brokes");
        println!("It was: {}", previous);
    });

    while rx.changed().await.is_ok() {
        println!("It came: {}", *rx.borrow());
    }
}

async fn demo_file_io() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::open("output.txt").await?;
    let mut content = String::new();

    file.read_to_string(&mut content).await?;
    println!("File content: {content}");

    let mut outfile = fs::File::create("newfile.txt").await?;
    outfile.write_all(content.as_bytes()).await?;

    Ok(())
}

async fn demo_process_sort() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = process::Command::new("sort");
    cmd.stdout(std::process::Stdio::piped());
    cmd.stdin(std::process::Stdio::piped());

    let mut child = cmd.spawn()?;
    let animals = &["dog", "cat", "bird", "frog", "duck", "enot"];

    let mut stdin = child.stdin.take().expect("child did have a handle to stdin");
    stdin.write(animals.join("\n").as_bytes()).await?;
    drop(stdin);

    let op = child.wait_with_output().await?;
    println!("Sorted:\n\n {}", std::str::from_utf8(&op.stdout)?);

    Ok(())
}

async fn demo_ctrl_c() -> Result<(), Box<dyn std::error::Error>> {
    println!("wait ctrl-c");
    signal::ctrl_c().await?;
    println!("received ctrl-c event");
    Ok(())
}

async fn demo_interval_ticks() {
    let duration = Duration::from_secs(2);
    let mut when = time::interval(duration);

    for _ in 0..3 {
        when.tick().await;
        println!("Tick!");
    }
}

async fn demo_timeout_sleepy() {
    if let Err(_) = time::timeout(Duration::from_secs_f32(3.33), sleepy()).await {
        log::info!("Time out");
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    demo_oneshot().await;
    demo_watch_multiple_receivers().await;
    demo_watch_send_replace().await;
    demo_file_io().await?;
    demo_process_sort().await?;
    demo_ctrl_c().await?;
    demo_interval_ticks().await;
    demo_timeout_sleepy().await;

    Ok(())
}
