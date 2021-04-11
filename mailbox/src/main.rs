use redisish::{parse, Command};
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use thiserror::Error;
#[derive(Error, Debug)]
pub enum MailBoxError {
    #[error("Could not parse")]
    ParseError(#[from] redisish::Error),
    #[error("Buffer error")]
    IoError(#[from] io::Error),
}

type Result<T> = std::result::Result<T, MailBoxError>;
struct ParseResult {
    command: Command,
    stream: TcpStream,
}
pub fn main() -> Result<()> {
    let mut vector: VecDeque<String> = VecDeque::new();
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let mut result = handle_client_command(stream?)?;
        match result.command {
            Command::Publish(msg) => vector.push_front(msg),
            Command::Retrieve => {
                if let Some(msg) = vector.back() {
                    writeln!(result.stream, "{}", msg)?;
                    vector.pop_back();
                }
            }
        }
    }
    Ok(())
}

fn handle_client_command(mut stream: TcpStream) -> Result<ParseResult> {
    let mut buffer = String::new();

    stream.read_to_string(&mut buffer)?;

    println!("string: {}", buffer);

    let command = parse(buffer.as_str())?;
    Ok(ParseResult { command, stream })
}
