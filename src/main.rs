use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:554").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected");

                let mut writer = BufWriter::new(&stream);
                writer
                    .write_all("RTSP/1.0 200 OK\r\n".as_bytes())
                    .expect("could not write");
                writer.flush().expect("could not flush");

                let mut reader = BufReader::new(&stream);
                let mut response = String::new();
                reader.read_line(&mut response).expect("could not read");
                println!("Server received {}", response);
            }
            Err(e) => {
                /* connection failed */
                println!("connection failed!");
            }
        }
    }
}
