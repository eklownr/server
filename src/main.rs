use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_stream(mut stream: TcpStream) -> Result<(), Error> {
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received request: {}", request);

        let response = "HTTP/1.1 200 OK\r
                        \r<!DOCTYPE html><html><body><h1>
                        Hello, from rust server!</h1></body></html>";
        stream.write_all(response.as_bytes())?;
        stream.flush()?;
    }
}

fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        handle_stream(stream?)?;
    }
    Ok(())
}
