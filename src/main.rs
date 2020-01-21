mod rtsp_msg_handler;
mod rtsp_session;
mod video_server;
use rtsp_msg_handler::{RtspCommand, RtspMessage, RtspParsable, RtspResponse};
use rtsp_session::{ClientPorts, RtspSession};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

fn handle_client(stream: TcpStream) {
    let client_ip = stream.peer_addr().unwrap().ip().to_string();
    println!("Client connected: {}", client_ip.to_owned());
    let mut reader = BufReader::new(&stream);
    let mut tcp_msg_buf = String::new();
    let mut session: Option<RtspSession> = None;

    loop {
        match reader.read_line(&mut tcp_msg_buf) {
            Ok(size) => {
                if size <= 0 {
                    break;
                }
                if tcp_msg_buf.contains("\r\n\r\n") {
                    let _string = str::from_utf8(&tcp_msg_buf.as_bytes()).unwrap();
                    println!("Request {:?}", _string);

                    match RtspMessage::parse_as_rtsp(tcp_msg_buf.to_owned()) {
                        Some(req) => {
                            match req.response() {
                                Some(resp) => {
                                    println!("Response {:?}\n", resp);
                                    let mut writer = BufWriter::new(&stream);
                                    match writer.write_all(resp.as_bytes()) {
                                        Ok(_) => (),
                                        Err(e) => (eprintln!("Error writing bytes: {}", e)),
                                    }
                                }
                                None => {
                                    eprintln!("No response found!");
                                    break;
                                }
                            }
                            match req.command {
                                Some(RtspCommand::Setup) => {
                                    session = Some(RtspSession::record(
                                        req.client_rtp.unwrap(),
                                        req.client_rtcp.unwrap(),
                                    ));
                                }
                                Some(RtspCommand::Play) => {
                                    let serve = video_server::serve_rtp(
                                        client_ip.to_owned(),
                                        session.as_ref().unwrap().client_rtp.as_ref().unwrap().to_owned(),
                                        session.as_ref().unwrap().client_rtcp.as_ref().unwrap().to_owned(),
                                        "5700".to_string(),
                                    );
                                    thread::spawn(move || serve);
                                }
                                Some(_) => (),
                                None => {
                                    eprintln!("Could not determine the Rtsp Command!");
                                    break;
                                }
                            }
                        }
                        None => {
                            eprintln!("Could not parse RtspMessage!");
                            break;
                        }
                    }
                    tcp_msg_buf.clear();
                }
            }
            Err(e) => {
                println!("Error reading TcpStream: {:?}", e);
                break;
            }
        }
    }
    println!("Client handled");
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:554").unwrap();
    println!("Server listening on port 554");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
