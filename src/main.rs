mod janestreet_parser;
use std::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main] // This makes main async
async fn main() {

    let (sender, receiver) = mpsc::channel::<String>();
    janestreet_parser::track_janestreet(sender).await;
    print!("teste")
}
