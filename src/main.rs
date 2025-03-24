mod janestreet_parser;
use std::sync::mpsc;
struct User {
    name: &'static str,
    contact: &'static str,
    handle: &'static str,
}

// Define a constant array and use it to initialize a Vec
const USERS: [User; 2] = [
    User {
        name: "Miguel Barbosa Pereira",
        contact: "alice@example.com",
        handle: "@alice",
    },
    User {
        name: "Pedro Pereira",
        contact: "bob@example.com",
        handle: "@bob",
    },
];



#[tokio::main] // This makes main async
async fn main() {

    let (sender, _receiver) = mpsc::channel::<String>();
    janestreet_parser::track_janestreet(sender).await;
    print!("teste")
}
