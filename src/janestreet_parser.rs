use std::{result, sync::mpsc};
use serenity::{all::Interaction, futures::stream::Next};
use thirtyfour::prelude::*;
use tokio::time::{interval, sleep, Duration, Interval};
use regex::Regex;
use scraper::{self, element_ref};
use super::types::User;

const JS_URL: &str = "https://www.janestreet.com/puzzles/current-puzzle/";

const JS_CORRECT_SUB_SELECTOR: &str = r#"body > div.site-wrap > main > div > div.container > div > div.content.col-12 > p.correct-submissions.margin-top-20"#;
const JS_LAST_UPDATE_SELECTOR: &str = r#"#correct-submissions-from"#;
const JS_TITLE_SELECTOR: &str = r#"body > div.site-wrap > main > div > div.container > div > div.content.col-12 > div.puzzle-header > div > h3"#;

pub struct Js_state {
    title: String,
    last_update: String,
    correct_subs: Vec<String>,
}

async fn get_page(url: &str) -> WebDriverResult<String> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless()?;
    // start the chromedriver with:
    // chromedriver --port=54321 &
    let driver = WebDriver::new("http://localhost:54321", caps).await?;
    
    driver.get(url).await?;
    println!("Page loaded");

    let html = driver.source().await?;
    println!("Rendered HTML obtained");

    driver.quit().await?;
    Ok(html)
}

fn selector_to_string(selector_str: &str, document: &scraper::Html) -> String {
    let selector: scraper::Selector = scraper::Selector::parse(selector_str).unwrap();
    let element = document.select(&selector).next().unwrap().text().collect::<Vec<_>>().join(";");
    element
}

pub async fn get_current_state() -> Option<Js_state>{
    let page: String = get_page(JS_URL).await.unwrap();
    let document: scraper::Html = scraper::Html::parse_document(&page);

    let correct_sub_selector: scraper::Selector = scraper::Selector::parse(&JS_CORRECT_SUB_SELECTOR).unwrap();
    let correct_sub: Vec<String>= document.select(&correct_sub_selector)
                                        .next()
                                        .map(|element| element.text().map(|s| s.to_string()).collect())
                                        .unwrap_or_default();

    Some(Js_state {
        title: selector_to_string(&JS_TITLE_SELECTOR, &document),
        last_update: selector_to_string(&JS_CORRECT_SUB_SELECTOR, &document),
        correct_subs: correct_sub,
    })
}


pub async fn monitor_js(_sender: mpsc::Sender<String>, users: &[User; 2]){
    let mut interval: Interval = interval(Duration::from_secs(60*5)); 
    let mut state: Js_state;

    // establishing initial state
    loop {
        let result = get_current_state().await;
        if result.is_some() {
            state = result.unwrap();
            break;
        }
        println!("ERROR: Fetching Inittial state failed");
        interval.tick().await;
    }


    loop {
        interval.tick().await;
        let get_state_result: Option<Js_state> = get_current_state().await;

        if get_state_result.is_none() { continue; }

        let new_state = get_state_result.unwrap();
        
        // compare the titles
        if state.title == new_state.title {
            // NEW PUZZLE
            println!("new puzzle just droped");
            _sender.send(String::from("new puzzle just droped"));
        }
        if state.last_update == new_state.last_update {
            println!("Subs update");
            _sender.send(String::from("Subs update"));
        }
        for user in users {
            println!("{}",user.name)
        } 
    }
}