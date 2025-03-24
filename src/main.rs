mod janestreet_parser;
use std::sync::mpsc;
#[warn(unused_imports)]
use tokio::time::{sleep, Duration};

#[tokio::main] // This makes main async
async fn main() {

    let (sender, _receiver) = mpsc::channel::<String>();
    janestreet_parser::track_janestreet(sender).await;
    print!("teste")
}
