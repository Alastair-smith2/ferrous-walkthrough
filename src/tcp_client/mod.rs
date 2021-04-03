use std::net::TcpStream;

pub fn create_client(port: String) -> std::io::Result<TcpStream> {
    let address = format!("127.0.0.1:{}", port);
    let stream = TcpStream::connect(address)?;
    Ok(stream)
}
