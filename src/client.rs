//! This library contains the methods and struct used to handle the connection
//! with the server, and communicate between it and the user. It contains
//! methods for connecting to, sending to, and receiving from the server, which
//! is the entirety of the logic for the client.
//! # Authors:
//! - Preston Knepper
//! - Richard Prange
//! # Version
//! - 12/6/24

use std::error::Error;
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use std::{process, thread};
use std::fmt::format;
use iced::Subscription;
use crate::battleship::{Message, Tile};
use crate::settings::Settings;


/// Handles the connection to the server.
///
#[derive(Debug)]
pub struct ConnectionHandler {
    /// Whether the server is connected.
    connected: bool,
    connection: TcpStream,
    pub username: String,
}


impl Clone for ConnectionHandler {
    fn clone(&self) -> Self {
        ConnectionHandler{
            connection:  self.connection.try_clone().unwrap(),
            connected: self.connected.clone(),
            username: self.username.clone()
        }
    }
}

impl ConnectionHandler {
    pub fn connect() -> Self {
        let settings = Settings::new();
        let mut send_stream = match TcpStream::connect(settings.host.clone() + ":" + &settings.port) {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1)
            }
        };


        send_stream.write_all(format!("/cnct {}", settings.username.clone() ).as_bytes()).unwrap();
        
        ConnectionHandler{
            connection: send_stream,
            connected: true,
            username: settings.username.clone(),
        }
    }


    pub fn fire(&mut self, t: Tile, username: String ){
        match self.connection.write_all(format!("/fire {} {} {}", &username, t.row, t.column).as_bytes()) {
            Ok(_) => {

            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }

    pub fn leave(){

    }


    pub fn start(&mut self){
        match self.connection.write_all("/start".as_bytes()) {
            Ok(_) => {

            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }
    
}




/// Makes connection to the game server and sends user input to server.
/// # Parameters
/// - `stream` - The 'TcpStream' wrapped in an `Arc<Mutex<?>>` which will send
///              the user's messages to the server.
/// - `username` - The name the user wishes to identify themselves as. It will
///                be shown to other players.
/// - `connection_handler` - A mutex which determines if the function should quit.
///                    Another thread will change this value if the loop should
///                    be aborted.
/// # Returns
/// This function only returns if an error occurs while sending messages.
fn send(
    mut stream: TcpStream,
    username: String,
    connection_handler: Arc<Mutex<ConnectionHandler>>,
) -> io::Result<()> {
    // send connect
    TcpStream::write_all(&mut stream, format!("/cnct {username}\n").as_bytes())
        .expect("Unable to connect to server");
    let mut input = String::new();
    while connection_handler.lock().unwrap().connected {
        input.clear();
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;
        println!();
    }
    Ok(())
}

/// Receives and handles messages from the server.
/// # Parameters
/// - `stream` - The `TcpStream` which will listen for messages.
/// - `connection_handler` - The handler which communicates the connection
///                          status between threads. Will be changed to false
///                          if connection is broken with the server.
/// # Commands
/// This listener only responds to a few commands which the server should send
/// in the correct format as listed bullets:
/// - `\msg <message>` - Upon receiving this command, the contents of
///                      `<message>` will print to stdout.
/// - `\alrt <message>` - Marks important actions which change the number of
///                       players (e.g. `Bob connected to the game`). The
///                       contents of `<message>` will print to stdout
///                       preceded by `!!!`.
/// - `\dscnct <message>` - Orders this client to disconnect from the server,
///                         thus leaving the game. Before the client
///                         disconnects, it will print the contents of
///                         `<message>` to stdout. This message should give a
///                         reason for the disconnection (e.g.
///                         `Game Over. You Lose!`).
/// # Returns
/// This function only returns if an error occurs while receiving messages.
fn listen(
    mut stream: TcpStream,
    connection_handler: Arc<Mutex<ConnectionHandler>>,
) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    // Breaks when the server sends the command to disconnect from the client,
    // or when the server otherwise disconnects from the client without reason.
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            println!("Server Disconnected.");
            break;
        }
        let received_message = String::from_utf8(Vec::from(&buffer[..bytes_read]))?;
        let index = first_word(&received_message);
        let command: &str = &received_message[0..index].trim().to_owned();
        let message: &str = &received_message[(index + 1)..received_message.len()].to_owned();
        match command {
            "/msg" => print!("\r{message}\n\n\r"),
            "/alrt" => print!("\r!!! {message}\n\n\r"),
            "/dscnct" => {
                println!("\r{message}\n");
                stream.shutdown(Shutdown::Both).unwrap();
                process::exit(0);
            }
            // Should not happen
            command => eprintln!("Received invalid command, {command}, from server.\n"),
        }
    }
    println!("Press enter to exit.");
    connection_handler.lock().unwrap().connected = false;
    Ok(())
}

/// Helper function for `listen` which retrieves the index of the end of the
/// first word in a string.
/// # Parameter
/// - `string` - The string to find the first word of.
/// # Returns
/// The index of the space following the first word in the string.
fn first_word(string: &String) -> usize {
    let bytes = string.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    string.len()
}


pub fn join() -> Subscription<Message> {
    Subscription::batch(vec![])
}

fn connect(){
    
}