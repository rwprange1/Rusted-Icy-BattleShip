/// # Overview:
/// This is a file which will be used to parse and set up the users
/// settings data, it will be used to connect to the server
/// 
/// # Authors:
/// - Richard Prange
/// # Version: 7/31/2025


use std::fs::{OpenOptions};
use serde::{Deserialize, Serialize};
use std::io::{Read};

#[derive(Serialize, Deserialize)]
pub struct Settings{
    pub username: String,
    pub host: String,
    pub port: String,
}

impl Settings {
    pub fn new() -> Settings {
        if let Ok(mut file) = OpenOptions::new().read(true).open("settings.json") {
            let mut buff = String::new();
            file.read_to_string(&mut buff).unwrap();
            let v: Settings = serde_json::from_str(&buff).unwrap();

            println!("IP: {}, Port: {}, Username: {}", v.host, v.port, v.username);

            return v;
        }
        panic!("Could not open settings.json");
    }
}
