use crate::rtsp_msg_parse::RawMessage;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
mod rtsp_msg_parse;
use rtsp_msg_parse::RtspRequest;

fn handle_client(stream: TcpStream) {
    println!("Client connected");

    let mut reader = BufReader::new(&stream);
    let mut data = String::new();
    loop {
        match reader.read_line(&mut data) {
            Ok(size) => {
                println!("{}", size);
                if size <= 0 {
                    break;
                }
                if data.contains("\r\n\r\n") {
                    let _string = str::from_utf8(&data.as_bytes()).unwrap();
                    println!("{}", _string);
                    let _parsed_req = RtspRequest::parse_as_rtsp(data.to_owned());
                    let _response = _parsed_req.unwrap().response().unwrap();
                    println!("Response {:?}", _response);
                    let mut writer = BufWriter::new(&stream);
                    writer
                        .write_all(_response.as_bytes())
                        .expect("could not write");
                    data.clear();
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }

    println!("Client handled");
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
