use crate::rtsp_req_parse::RawMessage;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
mod rtsp_req_parse;
use rtsp_req_parse::RtspRequest;

fn handle_client(stream: TcpStream) {
    println!("Client connected");

    let mut reader = BufReader::new(&stream);
    let mut data = String::new();
    loop {
        match reader.read_line(&mut data) {
            Ok(size) => {
                println!("{}", size);
                if size <= 2 {
                    break;
                }
                let _string = str::from_utf8(&data.as_bytes()).unwrap();
                println!("{}", _string);
            }
            Err(_) => {
                println!("Error");
                break;
            }
        }
    }
    let _parsed_req = RtspRequest::parse_as_rtsp(data);

    let mut writer = BufWriter::new(&stream);
    writer.write_all("\n".as_bytes()).expect("could not write");
    writer.flush().expect("could not flush");
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
