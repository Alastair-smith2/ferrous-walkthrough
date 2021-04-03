use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    let mut buffer = String::new();

    stream.read_to_string(&mut buffer)?;

    println!("string: {}", buffer);

    stream.write(buffer.as_bytes())?;

    Ok(())
}
