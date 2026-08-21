<<<<<<< HEAD
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Serialize, Deserialize, Debug)]
struct ToDo{
    id: u32,
    userId: u32,
    title: String,
    completed: bool,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let response = reqwest::get("https://httpbin.org/ip").await?;
    let ip: serde_json::Value = response.json().await?;
    println!("My IP: {}", ip);

    println!("Starting get tasks");

    match get_todo(1).await{
        Ok(todo) => println!("Task {} got", todo.id),
        Err(e) => println!("It's error: {}", e),
    }
    match get_user_todos(1).await{
        Ok(_) => println!("List has been received"),
        Err(e) => println!("Error; {}", e),
    }
    
    match check_user_tasks(4).await{
        Ok(_) => println!("Task statistics"),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

async fn get_todo(todo_id: u32) -> Result<ToDo, Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    let response: ToDo = client.get(format!("https://jsonplaceholder.typicode.com/todos/{}",todo_id))
        .send()
        .await?
        .json()
        .await?;

    println!("Task got");
    println!("ID: {}, user: {}, title: {}", response.id, response.userId, response.title);
    println!("Task completed: {}", 
        if response.completed {"Yes"} else {"Not today"});
    Ok(response)
}

async fn get_user_todos(user_id: u32) -> Result<(), Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    let response: Vec<ToDo> = client.get(&format!("https://jsonplaceholder.typicode.com/todos?userId={}", user_id))
    .send()
    .await?
    .json()
    .await?;

    println!("User {} tasks:", user_id);
    for todo in response.iter().take(5){
        println!("[{}] {}", 
            if todo.completed {"x"} else {" "},
            todo.title
        );
    }
    Ok(())
}

async fn check_user_tasks(user_id: u32) -> Result<(), Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    let response: Vec<ToDo> = client.get(&format!("https://jsonplaceholder.typicode.com/todos?userId={}", user_id))
    .send()
    .await?
    .json()
    .await?;

    let total = response.len();
    let completed = response.iter().filter(|t| t.completed).count();

    println!("User {} staticstic:", user_id);
    println!("All tasks: {}", total);
    println!("Completed tasks: {}", completed);
    println!("Remaining tasks: {}", total - completed);
    Ok(())
=======
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Serialize, Deserialize, Debug)]
struct ToDo{
    id: u32,
    userId: u32,
    title: String,
    completed: bool,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let response = reqwest::get("https://httpbin.org/ip").await?;
    let ip: serde_json::Value = response.json().await?;
    println!("My IP: {}", ip);

    println!("Starting get tasks");

    match get_todo(1).await{
        Ok(todo) => println!("Task {} got", todo.id),
        Err(e) => println!("It's error: {}", e),
    }
    match get_user_todos(1).await{
        Ok(_) => println!("List has been received"),
        Err(e) => println!("Error; {}", e),
    }
    
    match check_user_tasks(4).await{
        Ok(_) => println!("Task statistics"),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

async fn get_todo(todo_id: u32) -> Result<ToDo, Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    let response: ToDo = client.get(format!("https://jsonplaceholder.typicode.com/todos/{}",todo_id))
        .send()
        .await?
        .json()
        .await?;

    println!("Task got");
    println!("ID: {}, user: {}, title: {}", response.id, response.userId, response.title);
    println!("Task completed: {}", 
        if response.completed {"Yes"} else {"Not today"});
    Ok(response)
}

async fn get_user_todos(user_id: u32) -> Result<(), Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    let response: Vec<ToDo> = client.get(&format!("https://jsonplaceholder.typicode.com/todos?userId={}", user_id))
    .send()
    .await?
    .json()
    .await?;

    println!("User {} tasks:", user_id);
    for todo in response.iter().take(5){
        println!("[{}] {}", 
            if todo.completed {"x"} else {" "},
            todo.title
        );
    }
    Ok(())
}

async fn check_user_tasks(user_id: u32) -> Result<(), Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    let response: Vec<ToDo> = client.get(&format!("https://jsonplaceholder.typicode.com/todos?userId={}", user_id))
    .send()
    .await?
    .json()
    .await?;

    let total = response.len();
    let completed = response.iter().filter(|t| t.completed).count();

    println!("User {} staticstic:", user_id);
    println!("All tasks: {}", total);
    println!("Completed tasks: {}", completed);
    println!("Remaining tasks: {}", total - completed);
    Ok(())
>>>>>>> 7c0237918086acb0a596b6ab71be8a0a3a57ea44
}