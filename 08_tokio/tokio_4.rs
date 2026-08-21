use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{self, Command};
use std::process::Stdio;
use std::collections::VecDeque;

async fn sort_word() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut cmd = process::Command::new("sort");
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn()?;
    let mut child_stdin = child.stdin.take().expect("no stdin handle");
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    while let Some(line) = reader.next_line().await? {
        if line == "end" {
            break;
        }
        child_stdin.write_all(line.as_bytes()).await?;
        child_stdin.write_all(b"\n").await?;
    }
    drop(child_stdin);

    let op = child.wait_with_output().await?;
    Ok(std::str::from_utf8(&op.stdout)?.to_string())
}

async fn sort_num(input: Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error + Send + Sync>> {
    let owned_strs: Vec<String> = input.iter().map(|n| n.to_string()).collect();
    let nums_str: Vec<&str> = owned_strs.iter().map(|s| s.as_str()).collect();

    let mut cmd = process::Command::new("sort");
    cmd.arg("-n");
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn()?;
    let mut stdin = child.stdin.take().expect("no stdin handle");

    stdin.write_all(nums_str.join("\n").as_bytes()).await?;
    drop(stdin);
    let op = child.wait_with_output().await?;

    let text = std::str::from_utf8(&op.stdout)?;
    let results: Result<Vec<i32>, _> = text
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i32>())
        .collect();
    Ok(results?)
}

async fn tail_via_process(path: &str, n: usize) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let output = Command::new("tail")
        .arg("-n")
        .arg(n.to_string())
        .arg(path)
        .output()
        .await?;

    Ok(std::str::from_utf8(&output.stdout)?.to_string())
}

async fn tail_in_rust(path: &str, n: usize) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    let mut buffer: VecDeque<String> = VecDeque::with_capacity(n);
    for line in contents.lines() {
        if buffer.len() == n {
            buffer.pop_front();
        }
        buffer.push_back(line.to_string());
    }

    Ok(buffer.into_iter().collect())
}

async fn follow_via_process(path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut child = Command::new("tail")
        .arg("-f")
        .arg(path)
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().expect("no stdout handle");
    let mut reader = BufReader::new(stdout).lines();

    while let Some(line) = reader.next_line().await? {
        println!("[new line] {}", line);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let nums = vec![1, 221, 65, 6, 4, 84, 98, 4, 987, 98, 877, -7, 955, -1, 324];

    let word_handle = tokio::spawn(sort_word());
    let res_word = word_handle.await??;
    println!("Sorted word: {res_word}");

    let res_num = sort_num(nums).await?;
    println!("Sorted num: {:?}", res_num);

    match tail_via_process("app.log", 10).await {
        Ok(res) => println!("Tail via process:\n{res}"),
        Err(e) => println!("Tail via process error: {e}"),
    }

    match tail_in_rust("app.log", 10).await {
        Ok(lines) => println!("Tail in Rust:\n{}", lines.join("\n")),
        Err(e) => println!("Tail in Rust error: {e}"),
    }

    follow_via_process("server.log").await?;

    Ok(())
}