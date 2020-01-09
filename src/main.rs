use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
mod rtsp_msg_parse;
use rtsp_msg_parse::{RtspCommand, RtspParsable, RtspRequest};
mod video_server;

fn handle_client(stream: TcpStream) {
    println!("Client connected");

    let mut reader = BufReader::new(&stream);
    let mut data = String::new();
    let mut session_rtp1 = String::new();
    let mut session_rtp2 = String::new();

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
                    if _parsed_req.as_ref().unwrap().transport.as_ref().unwrap().0.to_owned() != "unset"
                    {
                        session_rtp1 = _parsed_req.as_ref().unwrap().transport.as_ref().unwrap().0.to_owned();
                        session_rtp2 = _parsed_req.as_ref().unwrap().transport.as_ref().unwrap().1.to_owned();
                    }
                    let _response = _parsed_req.as_ref().unwrap().response().unwrap();
                    println!("Response {:?}", _response);
                    let mut writer = BufWriter::new(&stream);
                    writer
                        .write_all(_response.as_bytes())
                        .expect("could not write bytes");
                    match _parsed_req.as_ref().unwrap().command {
                        Some(RtspCommand::Play) => {
                            video_server::serve_rtp(
                                "127.0.0.1".to_string(),
                                session_rtp1.to_owned(),
                                session_rtp2.to_owned(),
                                "5700".to_string(),
                            );
                        }
                        Some(_) => (),
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
