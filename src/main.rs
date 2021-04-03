mod tcp_echo;

use std::io;
use std::net::TcpListener;
use tcp_echo::handle_client;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
