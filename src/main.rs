mod rtsp_msg_handler;
mod rtsp_session;
mod video_server;
use rtsp_msg_handler::{RtspCommand, RtspMessage, RtspParsable, RtspResponse};
use rtsp_session::{ClientPorts, RtspSession};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

fn respond_to_client(req: RtspMessage, stream: &TcpStream, session: Option<RtspSession>) {
    match req.response(session) {
        Some(resp) => {
            println!("Response {:?}\n", resp);
            let mut writer = BufWriter::new(stream);
            match writer.write_all(resp.as_bytes()) {
                Ok(_) => (),
                Err(e) => (println!("Error writing bytes: {}", e)),
            }
        }
        None => {
            println!("No response found!");
        }
    }
}

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
                        Some(req) => match req.command {
                            Some(RtspCommand::Setup) => {
                                session = Some(RtspSession::setup(req.clone()));
                                respond_to_client(req.clone(), &stream, session.clone());
                            }
                            Some(RtspCommand::Play) => match session.clone() {
                                Some(_sess) => {
                                    let serve = |sess: RtspSession, client_ip: String| {
                                        video_server::serve_rtp(
                                            client_ip.clone(),
                                            sess.clone().client_rtp,
                                            sess.clone().client_rtcp,
                                            sess.clone().server_rtcp,
                                        )
                                    };
                                    let c_ip = client_ip.clone();
                                    thread::spawn(move || serve(_sess, c_ip));
                                    respond_to_client(req.clone(), &stream, session.clone());
                                    println!("Playing!");
                                }
                                None => {
                                    println!("No Session Found!");
                                    break;
                                }
                            },
                            Some(_) => {
                                respond_to_client(req.clone(), &stream, session.clone());
                            }
                            None => {
                                println!("Could not determine the Rtsp Command!");
                                break;
                            }
                        },
                        None => {
                            println!("Could not parse RtspMessage!");
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
