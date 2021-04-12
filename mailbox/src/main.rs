use std::collections::VecDeque;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum MailBoxError {
    #[error("Could not parse")]
    ParseError(#[from] redisish::Error),
    #[error("Buffer error")]
    IoError(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, MailBoxError>;
pub fn main() -> Result<()> {
    let storage: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for connection in listener.incoming() {
        let stream = match connection {
            Ok(stream) => stream,
            Err(e) => {
                println!("Error occured: {:?}", e);
                continue;
            }
        };
        let storage_clone = Arc::clone(&storage);
        let handler = thread::spawn(move || {
            let mut vec_storage = storage_clone.lock().unwrap();
            let res = handle(stream, &mut *vec_storage);

            if let Err(e) = res {
                println!("Error occured: {:?}", e);
            }
        });
        handler.join().unwrap();
    }

    Ok(())
}

fn handle(mut stream: TcpStream, storage: &mut VecDeque<String>) -> Result<()> {
    let command = read_command(&mut stream)?;

    match command {
        redisish::Command::Publish(message) => {
            storage.push_back(message);
        }
        redisish::Command::Retrieve => {
            let data = storage.pop_front();

            match data {
                Some(message) => write!(stream, "{}", message)?,
                None => write!(stream, "No message in inbox!\n")?,
            }
        }
    }
    Ok(())
}

fn read_command(stream: &mut TcpStream) -> Result<redisish::Command> {
    let mut read_buffer = String::new();
    let mut buffered_stream = BufReader::new(stream);
    buffered_stream.read_line(&mut read_buffer)?;
    let command = redisish::parse(&read_buffer)?;
    Ok(command)
}
