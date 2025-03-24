use std::sync::mpsc;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

const JS_URL: &str = "https://www.janestreet.com/puzzles/current-puzzle/";

async fn get_page(url: &str) -> WebDriverResult<String> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless()?;

    let driver = WebDriver::new("http://localhost:42631", caps).await?;
    
    driver.get(url).await?;
    println!("Page loaded");

    sleep(Duration::from_secs(5)).await; // Wait for rendering
    println!("After sleep");

    let html = driver.source().await?;
    println!("Rendered HTML obtained");

    driver.quit().await?;
    Ok(html)
}

pub async fn track_janestreet(_sender: mpsc::Sender<String>){
    let page = get_page(JS_URL);
    print!("{:?}", page.await);
}