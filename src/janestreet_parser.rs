use std::sync::mpsc;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};
use regex::Regex;


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

fn get_submission_date(page: String) -> String {
    let re = Regex::new(r"Correct submissions as of (.*):").unwrap();
    if let Some(captures) = re.captures(&page) {
        if let Some(matched) = captures.get(1) {
            return matched.as_str().to_string()
        }
    }
    String::from("")
}

fn search_correct_submissions(names: Vec<String>, page: String) -> Vec<bool> {
    let mut has_correct_submission: Vec<bool> = Vec::new();
    for name in names {
        if page.contains(&name) {
            has_correct_submission.push(true);
        } else {
            has_correct_submission.push(false);
        }
    }
    has_correct_submission

}

pub async fn track_janestreet(_sender: mpsc::Sender<String>){
    let page = get_page(JS_URL).await;

    print!("{:?}", page);
}