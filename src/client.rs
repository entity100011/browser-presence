use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;

pub struct Client {
    discord: discord_rpc_client::Client,
}

impl Client {
    pub fn new(client_id: u64) -> Client {
        let mut n = discord_rpc_client::Client::new(client_id);
        n.start();
        Client { discord: n }
    }

    pub fn set_presence(&mut self, presence: Presence) -> Result<(), String> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => {
                match self.discord.set_activity(|act| {
                    act.state(presence.state)
                        .details(presence.details)
                        .timestamps(|ts| ts.start(n.as_secs()))
                        .assets(|ass| ass.large_image("image_large").large_text("OOOOOFFF"))
                }) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Failed to set presence: {}", e)),
                }
            }
            Err(_) => Err("Time has gone backwards".to_string()),
        }
    }

    pub fn clear_presence(&mut self) -> Result<(), String> {
        match self.discord.clear_activity() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to clear presence: {}", e)),
        }
    }
}

pub struct ClientManager {
    clients: HashMap<String, Client>,
}

impl ClientManager {
    pub fn new() -> ClientManager {
        let bytes = fs::read("clients.json").expect("Failed to read from file 'clients.json'. Does it exist? Do you have the required permissions to access it?");
        let json =
            String::from_utf8(bytes).expect("Invalid UTF-8 character found in file 'clients.json'");
        let clients_json: ClientsJSON =
            serde_json::from_str(&json).expect("Invalid JSON in file 'clients.json'");

        let mut clients: HashMap<String, Client> = HashMap::new();

        for (friendly_name, client_id) in clients_json.clients.iter() {
            let n = Client::new(*client_id);
            clients.insert(friendly_name.to_string(), n);
        }

        ClientManager { clients }
    }

    pub fn try_set_presence(&mut self, name: &str, presence: Presence) -> Result<(), String> {
        match self.clients.contains_key(name) {
            true => match self.clients.get_mut(name).unwrap().set_presence(presence) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            false => Err("Invalid client name".to_string()),
        }
    }

    pub fn try_clear_presence(&mut self, name: &str) -> Result<(), String> {
        match self.clients.contains_key(name) {
            true => match self.clients.get_mut(name).unwrap().clear_presence() {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            false => Err("Invalid client name".to_string()),
        }
    }

    pub fn try_clear_all(&mut self) -> Result<(), String> {
        let mut iter = self.clients.iter_mut();

        loop {
            match iter.next() {
                Some((_, client)) => {
                    match client.clear_presence() {
                        Ok(_) => continue,
                        Err(e) => break Err(e),
                    }
                }
                None => break Ok(()),
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClientsJSON {
    pub clients: HashMap<String, u64>,
}

pub struct Presence {
    pub state: String,
    pub details: String,
}
