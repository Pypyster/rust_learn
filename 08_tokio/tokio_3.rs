use futures::channel::oneshot;
use tokio::sync::{mpsc, watch};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::fs;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::fmt::Display;
use std::future::Future;

type AppResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

enum Action {
    Read(String, oneshot::Sender<AppResult<String>>),
    Reqwest(u32, oneshot::Sender<AppResult<String>>),
    Computer(Box<dyn Fn(u32) -> u64 + Send + Sync>, oneshot::Sender<AppResult<u64>>),
    Write(Vec<u8>, oneshot::Sender<AppResult<()>>),
}

async fn reader(path: &str) -> AppResult<String> {
    let file = fs::File::open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut result = String::new();
    let mut idx = 1;
    while let Some(line) = lines.next_line().await? {
        result.push_str(&format!("{idx} line: {line}\n"));
        idx += 1;
    }
    Ok(result)
}

async fn writer<T: AsRef<[u8]>>(content: T) -> AppResult<()> {
    let mut file = fs::File::create("output_res.txt").await?;
    file.write_all(content.as_ref()).await?;
    Ok(())
}

async fn get_post(client: &reqwest::Client, id: u32) -> AppResult<String> {
    let url = format!("https://jsonplaceholder.typicode.com/posts/{id}");
    let resp = client.get(&url).send().await?;
    Ok(resp.text().await?)
}

async fn fetch_all(client: &reqwest::Client) -> AppResult<String> {
    let ids = vec![1, 2, 3, 4, 5];
    let start = Instant::now();

    let tasks: Vec<_> = ids.into_iter().map(|id| {
        let client = client.clone();
        tokio::spawn(async move { get_post(&client, id).await })
    }).collect();

    let mut success = 0;
    let mut failed = 0;

    for task in tasks {
        match task.await? {
            Ok(_) => success += 1,
            Err(e) => {
                eprintln!("Error: {e}");
                failed += 1;
            }
        }
    }

    Ok(format!("Ready: {success} success, {failed} with error, took {:?}", start.elapsed()))
}

fn heavy_compute(id: u32) -> u64 {
    std::thread::sleep(Duration::from_secs(7));
    (id * id) as u64
}

fn update_status(active_tasks: &Arc<AtomicUsize>, status_tx: &watch::Sender<String>, starting: bool) -> usize {
    let count = if starting {
        active_tasks.fetch_add(1, Ordering::SeqCst) + 1
    } else {
        active_tasks.fetch_sub(1, Ordering::SeqCst) - 1
    };
    let status = if count == 0 { "Idle".to_string() } else { format!("Processing ({count} active)") };
    let _ = status_tx.send(status);
    count
}

async fn run_tracked<F, T>(
    active_tasks: Arc<AtomicUsize>,
    status_tx: watch::Sender<String>,
    label: &'static str,
    fut: F,
) -> T
where
    F: Future<Output = T>,
{
    update_status(&active_tasks, &status_tx, true);
    println!("[{label}] started");

    let result = fut.await;

    update_status(&active_tasks, &status_tx, false);
    println!("[{label}] finished");

    result
}

async fn report<T: Display>(rx: oneshot::Receiver<AppResult<T>>, label: &str) {
    match rx.await {
        Ok(Ok(value)) => println!("[{label}] result: {value}"),
        Ok(Err(e)) => eprintln!("[{label}] error: {e}"),
        Err(_) => eprintln!("[{label}] sender dropped"),
    }
}

async fn report_unit(rx: oneshot::Receiver<AppResult<()>>, label: &str) {
    match rx.await {
        Ok(Ok(())) => println!("[{label}] completed"),
        Ok(Err(e)) => eprintln!("[{label}] error: {e}"),
        Err(_) => eprintln!("[{label}] sender dropped"),
    }
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let client = reqwest::Client::new();
    let (sender, mut receiver) = mpsc::channel::<Action>(10);
    let (status_tx, status_rx) = watch::channel("Idle".to_string());
    let active_tasks = Arc::new(AtomicUsize::new(0));

    let (tx1, rx1) = oneshot::channel();
    sender.send(Action::Read("newfile.txt".to_string(), tx1)).await?;

    let (tx2, rx2) = oneshot::channel();
    sender.send(Action::Write("It's new test for writer".to_string().into_bytes(), tx2)).await?;

    let (tx3, rx3) = oneshot::channel();
    sender.send(Action::Reqwest(1, tx3)).await?;

    let (tx4, rx4) = oneshot::channel();
    sender.send(Action::Computer(Box::new(heavy_compute), tx4)).await?;

    drop(sender);

    let mut status_watcher = status_rx.clone();
    tokio::spawn(async move {
        while status_watcher.changed().await.is_ok() {
            println!("[STATUS] {}", *status_watcher.borrow());
        }
    });

    let processor = tokio::spawn(async move {
        while let Some(item) = receiver.recv().await {
            match item {
                Action::Read(path, tx) => {
                    let active_tasks = active_tasks.clone();
                    let status_tx = status_tx.clone();
                    tokio::spawn(async move {
                        let result = run_tracked(active_tasks, status_tx, "Read", reader(&path)).await;
                        let _ = tx.send(result);
                    });
                }
                Action::Write(content, tx) => {
                    let active_tasks = active_tasks.clone();
                    let status_tx = status_tx.clone();
                    tokio::spawn(async move {
                        let result = run_tracked(active_tasks, status_tx, "Write", writer(content)).await;
                        let _ = tx.send(result);
                    });
                }
                Action::Reqwest(id, tx) => {
                    let client = client.clone();
                    let active_tasks = active_tasks.clone();
                    let status_tx = status_tx.clone();
                    tokio::spawn(async move {
                        let fut = async {
                            match id {
                                0 => fetch_all(&client).await,
                                _ => get_post(&client, id).await,
                            }
                        };
                        let result = run_tracked(active_tasks, status_tx, "Reqwest", fut).await;
                        let _ = tx.send(result);
                    });
                }
                Action::Computer(func, tx) => {
                    let active_tasks = active_tasks.clone();
                    let status_tx = status_tx.clone();
                    tokio::spawn(async move {
                        let fut = async move {
                            tokio::task::spawn_blocking(move || func(42))
                                .await
                                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
                        };
                        let result = run_tracked(active_tasks, status_tx, "Computer", fut).await;
                        let _ = tx.send(result);
                    });
                }
            }
        }
    });

    report(rx1, "Read").await;
    report_unit(rx2, "Write").await;
    report(rx3, "Reqwest").await;
    report(rx4, "Computer").await;

    processor.await?;
    Ok(())
}