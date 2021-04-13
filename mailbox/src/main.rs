use std::collections::VecDeque;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

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
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let storage = VecDeque::new();
    let rced_storage: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(storage));

    for connection in listener.incoming() {
        let mut stream = match connection {
            Ok(stream) => stream,
            Err(e) => {
                println!("Error occured: {:?}", e);
                continue;
            }
        };

        let storage_handle = rced_storage.clone();

        std::thread::spawn(move || {
            let res = handle(&mut stream, &storage_handle);

            if let Err(e) = res {
                println!("Error occured: {:?}", e);
            }
        });
    }

    Ok(())
}

fn handle(stream: &mut TcpStream, mutex: &Mutex<VecDeque<String>>) -> Result<()> {
    let command = read_command(stream)?;

    match command {
        redisish::Command::Publish(message) => {
            let mut storage = mutex.lock().unwrap();
            storage.push_back(message);
        }
        redisish::Command::Retrieve => {
            let mut storage = mutex.lock().unwrap();
            let data = storage.pop_front();
            match data {
                Some(message) => write!(stream, "{}", message)?,
                None => write!(stream, "No message in inbox!\n")?,
            };
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
