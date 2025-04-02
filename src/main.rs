mod janestreet_parser;
mod types;
use serenity::model::id::UserId;
use std::fs;
use std::sync::mpsc;
use types::User;

// Define a constant array and use it to initialize a Vec
const USERS: [User; 2] = [
    User {
        name: "Miguel Barbosa Pereira",
        discord_id: 181514107218821131,
    },
    User {
        name: "Pedro Pereira",
        discord_id: 148209061207343104,
    },
];

async fn send_discord_message(user_id: u64, http: &serenity::http::Http, message: &String) {
    match UserId::new(user_id).create_dm_channel(&http).await {
        Ok(channel) => {
            if let Err(e) = channel.say(&http, message).await {
                eprintln!("Failed to send message: {:?}", e);
            } else {
                println!("Message sent successfully!");
            }
        }
        Err(e) => eprintln!("Failed to create DM channel: {:?}", e),
    }
}

// This makes main async
#[tokio::main]
async fn main() {
    // Read the bot token from the "token" file
    let token = match fs::read_to_string("/token") {
        Ok(t) => t.trim().to_string(), // Trim to remove any trailing newline
        Err(e) => {
            eprintln!("Failed to read token file: {}", e);
            return;
        }
    };

    let http = serenity::http::Http::new(&token);

    let (sender, _receiver) = mpsc::channel::<String>();
    tokio::spawn(async move {
        janestreet_parser::monitor_js(sender, &USERS).await;
    });

    // receive messages
    for received in _receiver {
        let mut parts = received.splitn(2, ';');
        let user_name = parts.next().unwrap_or("").to_string();
        let message = parts.next().unwrap_or("").to_string();
        for user in &USERS {
            if user_name == user.name {
                println!("{}: {}", user.discord_id, &message);
                send_discord_message(user.discord_id, &http, &message).await;
            }
        }
    }
}
