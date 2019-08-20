mod client;
mod util;

use util::*;
use client::{ClientManager, Presence};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;
use termion::style::Reset;
use ws::listen;

#[derive(Serialize, Deserialize)]
struct Request {
    pub name: Option<String>,
    pub state: Option<String>,
    pub details: Option<String>,
    pub clear: Option<bool>,
}

fn main() {
    let (s, r): (SyncSender<String>, Receiver<String>) = sync_channel(1);

    info("Setting up threads");
    thread::spawn(move || {
        let mut client_manager = ClientManager::new();

        for message in r {
            let request: Request = match serde_json::from_str(&message) {
                Ok(n) => n,
                Err(e) => {
                    error(format!("Error deserializing JSON: {}", e));
                    continue;
                }
            };

            match request.clear {
                None => {
                    match (request.name, request.details, request.state) {
                        (Some(name), Some(details), Some(state)) => {
                            match client_manager.try_set_presence(
                                &name,
                                Presence {
                                    state: state,
                                    details: details,
                                },
                            ) {
                                Ok(_) => {
                                    // Display shit
                                    info(format!("Setting presence for client '{}'", name));
                                }
                                Err(e) => error(&e),
                            };
                        }
                        _ => error("Invalid request! One of the required fields 'name', 'details', or 'state' is missing!"),
                    };
                }
                Some(n) => {
                    match n {
                        true => {
                            match request.name {
                                Some(name) => {
                                    match client_manager.try_clear_presence(&name) {
                                        Ok(_) => info("Clearing presence"),
                                        Err(e) => error(&e),
                                    };
                                }
                                None => {
                                    match client_manager.try_clear_all() {
                                        Ok(_) => info("Clearing presence"),
                                        Err(e) => error(&e),
                                    };
                                },
                            };
                        }
                        false => error("Bruh, you're supposed to set the 'clear' field to either null or true. False is an invalid choice, even if it is a boolean"),
                    };
                }
            };
        }
    });

    // Port 43069 ( ͡° ͜ʖ ͡°)
    info(format!("Starting server on port {}4{}30{}69{} ( ͡° ͜ʖ ͡°)", YELLOW, GREEN, RED, Reset));
    listen("127.0.0.1:43069", |out| {
        let s = s.clone();

        move |msg: ws::Message| {
            match msg.as_text() {
                Ok(n) => s.send(n.to_string()).expect("Channel disconnected"),
                Err(_) => println!("Found invalid UTF-8 characters in data"),
            };
            out.send(msg)
        }
    })
        .unwrap();
}

