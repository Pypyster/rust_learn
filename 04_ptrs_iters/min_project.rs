use reqwest::Error;
use scraper::{Html, Selector};
use tokio::task;

#[derive(Debug)]

struct ParseData{
    titles: Vec<String>,
    links: Vec<String>,
}

 async fn fetcy_html(url: String) -> Result<String, Error> {
    let response = reqwest::get(&url).await?; // переброс ожидания всех функций
    response.text().await
}

fn parse_html (html: &String) -> ParseData{
    let document = Html::parse_document(html);
    let title_selector = Selector::parse("h1, h2, h3").unwrap();
    let links_selektor = Selector::parse("a").unwrap();

    let titles = document.select(&title_selector)
        .map(|element| element.text().collect::<Vec<_>>().join(" ")).collect();

    let links = document.select(&links_selektor)
        .filter_map(|element| element.value().attr("href"))
        .map(String::from).collect();

    ParseData { titles, links }
}

async fn process_urls(urls: Vec<String>) {
    let mut handles = Vec::new();

    for url in urls {
        let handle = task::spawn(async move {
            match fetcy_html(url.clone()).await {
                Ok(html) => {
                    let data = parse_html(&html);
                    println!("Parsed data: {:#?}", data);
                }
                Err(e) => println!("Failed to fetch URl: {}", e),
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::main]
async fn main(){
    let urls = vec![
        "https://www.google.com/?zx=1784222618264".to_string(),
        "https://ruso-oge.sdamgia.ru/".to_string(),
        "https://www.rust-lang.org/".to_string(),
        "https://www.example.com/".to_string(),
        "https://www.google.com/search?q=%D1%87%D1%82%D0%BE+%D1%82%D0%B0%D0%BA%D0%BE%D0%B5+%D0%BF%D0%B0%D1%80%D1%81%D0%B8%D0%BD%D0%B3&oq=%D1%87%D1%82%D0%BE+%D1%82%D0%B0%D0%BA%D0%BE%D0%B5+%D0%BF%D0%B0%D1%80%D1%81&gs_lcrp=EgRlZGdlKgcIARAAGIAEMgYIABBFGDkyBwgBEAAYgAQyBwgCEAAYgAQyBwgDEAAYgAQyBwgEEAAYgAQyBwgFEAAYgAQyBwgGEAAYgAQyCAgHEAAYFhgeMggICBAAGBYYHtIBCTEyMDY0ajBqN6gCAbACAQ&sourceid=chrome&source=chrome.ob&ie=UTF-8".to_string(),
    ];

    process_urls(urls).await;
}