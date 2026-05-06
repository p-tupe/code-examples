use std::{env, time::Duration};

fn main() {
    let urls: Vec<String> = env::args().collect();
    trpl::block_on(async {
        if let Some(title) = page_title(&urls[1]).await {
            println!("Title is: {title}");
        }

        counters().await;
    });
}

async fn page_title(url: &str) -> Option<String> {
    let text = trpl::get(url).await.text().await;
    trpl::Html::parse(&text)
        .select_first("title")
        .map(|t| t.inner_html())
}

async fn counters() {
    let handler = trpl::spawn_task(async {
        for i in 1..10 {
            println!("hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the second task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }

    handler.await.unwrap();
}
