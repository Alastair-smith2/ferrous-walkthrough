mod tcp_client;

use std::{borrow::ToOwned, io};
use std::{io::prelude::*, net::Shutdown};
use tcp_client::create_client;

fn main() -> io::Result<()> {
    let arg = std::env::args().nth(1);

    let message = match arg {
        Some(msg) => msg,
        None => String::from("Default Message"),
    };
    println!("The message {:?}", &message);
    let mut client = create_client("8080".to_owned())?;
    client.write(message.as_bytes())?;
    let mut buffer = String::new();
    client.shutdown(Shutdown::Write)?;
    client.read_to_string(&mut buffer)?;
    println!("The result {:?}", buffer);
    Ok(())
}
