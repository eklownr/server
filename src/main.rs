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