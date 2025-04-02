use super::types::User;
use scraper::{self};
use std::io::{self, Write};
use std::sync::mpsc;
use thirtyfour::prelude::*;
use tokio::time::{Duration, Interval, interval};

const JS_URL: &str = "https://www.janestreet.com/puzzles/current-puzzle/";

const JS_CORRECT_SUB_SELECTOR: &str = r#"body > div.site-wrap > main > div > div.container > div > div.content.col-12 > p.correct-submissions.margin-top-20"#;
//const JS_LAST_UPDATE_SELECTOR: &str = r#"#correct-submissions-from"#;
const JS_TITLE_SELECTOR: &str = r#"body > div.site-wrap > main > div > div.container > div > div.content.col-12 > div.puzzle-header > div > h3"#;

pub struct JsState {
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
    let element = document
        .select(&selector)
        .next()
        .unwrap()
        .text()
        .collect::<Vec<_>>()
        .join(";");
    element
}

pub async fn get_current_state() -> Option<JsState> {
    let page: String = get_page(JS_URL).await.unwrap();
    let document: scraper::Html = scraper::Html::parse_document(&page);

    let correct_sub_selector: scraper::Selector =
        scraper::Selector::parse(JS_CORRECT_SUB_SELECTOR).unwrap();
    let correct_sub: Vec<String> = document
        .select(&correct_sub_selector)
        .next()
        .map(|element| element.text().map(|s| s.to_string()).collect())
        .unwrap_or_default();

    Some(JsState {
        title: selector_to_string(JS_TITLE_SELECTOR, &document),
        last_update: selector_to_string(JS_CORRECT_SUB_SELECTOR, &document),
        correct_subs: correct_sub,
    })
}

pub async fn monitor_js(_sender: mpsc::Sender<String>, users: &[User; 2]) -> ! {
    println!("entered monitor js");
    io::stdout().flush().unwrap();
    let mut interval: Interval = interval(Duration::from_secs(10));
    let mut state: JsState;

    // establishing initial state
    loop {
        let result = get_current_state().await;
        if result.is_some() {
            state = result.unwrap();
            println!("SUCCESS: Fetching Inittial state SUCCEDED");
            io::stdout().flush().unwrap();
            break;
        }
        println!("ERROR: Fetching Inittial state failed");
        interval.tick().await;
    }

    loop {
        interval.tick().await;
        let get_state_result: Option<JsState> = get_current_state().await;

        if get_state_result.is_none() {
            continue;
        }

        let new_state = get_state_result.unwrap();

        // compare the titles
        if state.title != new_state.title {
            // NEW PUZZLE
            println!("INFO: A new puzzle was posted");
            for user in users {
                let message = format!(
                    "{};New puzzle just droped: {}\n{} ",
                    user.name, state.title, JS_URL
                );
                println!("{}", message);
                let _ = _sender.send(message);
            }
            state = new_state;
            continue;
        }

        // compare last update
        if state.last_update == new_state.last_update {
            println!("INFO: No updates");
            state = new_state;
            continue;
        }

        // check for correct submissions
        for user in users {
            if !state.correct_subs.contains(&user.name.to_string())
                && new_state.correct_subs.contains(&user.name.to_string())
            {
                println!("INFO: {} solved the puzzle!🎉", user.name);
                let message = format!(
                    "{};Congratulations, you solved the puzzle!🎉\n{} ",
                    user.name, JS_URL
                );
                let _ = _sender.send(message);
            }
        }
        state = new_state;
    }
}
