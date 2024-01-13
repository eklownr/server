use std::net::{TcpListener, TcpStream};
use std::io::{Error, Read};

fn handle_stream(mut stream: TcpStream) -> Result<(), Error> {
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received request: {}", request);
    }
}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                if let Err(err) = handle_stream(stream) {
                    eprintln!("Error handling stream: {}", err);
                }
            }
            Err(err) => {
                eprintln!("Error accepting connection: {}", err);
            }
        }
    }
}


/*
use std::net::{TcpListener, TcpStream};
use std::io::{Error, Read};

fn main() {
    let listener: TcpListener =
        TcpListener::bind("127.0.0.1:7878").unwrap(); 

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");

                // Handle stream here
            }
            Err(err) => {
                eprintln!("Error accepting connection: {}", err);
            }
        }
    }
}
*/
