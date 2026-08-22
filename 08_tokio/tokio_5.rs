use rand::random_range;
use tokio::sync::broadcast;
use tokio::time::{Duration, interval};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::IntervalStream;

#[derive(Debug)]
enum Event {
    Random(i32),
    Tick,
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(16);

    for el in -7..3 {
        tx.send(el).unwrap();
    }
    let mut rx1 = tx.subscribe();

    for _ in 0..6 {
        let el = rand::random_range(-10000..=999);
        tx.send(el).unwrap();
    }

    drop(tx);

    let ear_th = tokio::spawn(async move {
        while let Ok(v) = rx.recv().await {
            println!("Receiver 1: {v}");
        }
        println!("The first thread is finished");
    });

    let lat_th = tokio::spawn(async move {
        while let Ok(v) = rx1.recv().await {
            println!("Receiver 2: {v}");
        }
        println!("The second thread is finished");
    });

    let _ = tokio::join!(ear_th, lat_th);

    let random_stream = IntervalStream::new(interval(Duration::from_secs(1)))
        .map(|_| Event::Random(random_range(-100..101)));

    let tick_stream = IntervalStream::new(interval(Duration::from_millis(400)))
        .map(|_| Event::Tick);

    let mut merged = random_stream.merge(tick_stream);
    let mut count = 0;

    while let Some(event) = merged.next().await {
        match event {
            Event::Random(n) => println!("Random num: {n}"),
            Event::Tick => println!("Tick-tack"),
        }
        count += 1;
        if count > 12 {
            break;
        }
    }
}