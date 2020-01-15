mod rtsp_msg_handler;
use rtsp_msg_handler::{RtspCommand, RtspMessage, RtspParsable, RtspResponse};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
mod video_server;

fn handle_client(stream: TcpStream) {
    println!("Client connected");

    let mut reader = BufReader::new(&stream);
    let mut data = String::new();
    let mut client_rtp_port: String = String::new();
    let mut client_rtcp_port: String = String::new();

    loop {
        match reader.read_line(&mut data) {
            Ok(size) => {
                if size <= 0 {
                    break;
                }
                if data.contains("\r\n\r\n") {
                    let _string = str::from_utf8(&data.as_bytes()).unwrap();
                    println!("Request {:?}", _string);

                    match RtspMessage::parse_as_rtsp(data.to_owned()) {
                        Some(req) => {
                            if let Some(port) = &req.client_rtp {
                                client_rtp_port = port.to_string();
                            }
                            if let Some(port) = &req.client_rtcp {
                                client_rtcp_port = port.to_string();
                            }
                            match req.response() {
                                Some(resp) => {
                                    println!("Response {:?}\n", resp);
                                    let mut writer = BufWriter::new(&stream);
                                    writer
                                        .write_all(resp.as_bytes())
                                        .expect("could not write bytes");
                                }
                                None => (),
                            }
                            match req.command {
                                Some(RtspCommand::Play) => {
                                    let a = client_rtp_port.clone();
                                    let b = client_rtcp_port.clone();
                                    thread::spawn(move || {
                                        video_server::serve_rtp(
                                            "127.0.0.1".to_string(),
                                            a.to_string(),
                                            b.to_string(),
                                            "5700".to_string(),
                                        );
                                    });
                                }
                                Some(_) => (),
                                None => (),
                            }
                        }
                        None => (),
                    }
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
