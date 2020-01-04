use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut data: Vec<u8> = [1u8; 1000].to_vec(); // using 50 byte buffer
    match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            let as_string = str::from_utf8(&data).unwrap();
            println!(
                "OK!, read {} bytes \n***********\n{}\n***********",
                size, as_string
            );
            stream.write_all(&data).unwrap();
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }
    println!("Client Handling Complete!");
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:554").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 554");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
