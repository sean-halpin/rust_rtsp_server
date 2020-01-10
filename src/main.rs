use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
mod rtsp_msg_handler;
use rtsp_msg_handler::{RtspCommand, RtspParsable, RtspRequest};
mod video_server;

fn handle_client(stream: TcpStream) {
    println!("Client connected");

    let mut reader = BufReader::new(&stream);
    let mut data = String::new();
    let mut client_rtp_port: Option<String> = None;
    let mut client_rtcp_port: Option<String> = None;

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

                    match RtspRequest::parse_as_rtsp(data.to_owned()) {
                        Some(req) => {
                            client_rtp_port = req.client_rtp.to_owned();
                            client_rtcp_port = req.client_rtcp.to_owned();
                            match req.response() {
                                Some(resp) => {
                                    println!("Response {:?}", resp);
                                    let mut writer = BufWriter::new(&stream);
                                    writer
                                        .write_all(resp.as_bytes())
                                        .expect("could not write bytes");
                                }
                                None => (),
                            }
                            match req.command {
                                Some(RtspCommand::Play) => {
                                    thread::spawn(move || {
                                        video_server::serve_rtp(
                                            "127.0.0.1".to_string(),
                                            client_rtp_port.unwrap_or("".to_string()).to_owned(),
                                            client_rtcp_port.unwrap_or("".to_string()).to_owned(),
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
