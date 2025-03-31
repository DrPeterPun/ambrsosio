mod janestreet_parser;
mod types;
use types::User;
use std::{thread, sync::mpsc};


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
    let sender2 = sender.clone();
    let _sender3 = sender.clone();
    thread::spawn(move || janestreet_parser::monitor_js( sender2, &USERS));
    //thread::spawn(move || janestreet_parser::monitor_js( sender3, &USERS));

    for received in _receiver {
        println!("yoyo receibe {}",received);
    }
    print!("teste")
}
