extern crate chrono;
use chrono::Utc;

pub enum RtspCommand {
    Options,
    Describe,
    Play,
    Setup,
    Teardown,
}

pub trait RawMessage {
    fn parse_as_rtsp(raw: String) -> Option<Self>
    where
        Self: std::marker::Sized;
    fn response(&self) -> Option<String>;
}

pub struct RtspRequest {
    command: Option<RtspCommand>,
    uri: Option<String>,
    rtsp_spec_version: Option<String>,
    cseq: Option<i32>,
    user_agent: Option<String>,
    session: Option<String>,
    transport: Option<String>,
}

fn rtsp_date_time() -> String {
    return "Date: ".to_owned() + &Utc::now().to_rfc2822();
}

impl RawMessage for RtspRequest {
    fn response(&self) -> Option<String> {
        let _header_ok = "RTSP/1.0 200 OK".to_owned();
        let mut _response_lines: Vec<String> = Vec::new();
        match self.command {
            Some(RtspCommand::Options) => {
                let _server_functions =
                    "Public: OPTIONS, DESCRIBE, PLAY, SETUP, TEARDOWN".to_owned();
                let _server_id = "Server: Rust RTSP server".to_owned();
                _response_lines.push(_header_ok);
                _response_lines.push("CSeq: ".to_owned() + &(self.cseq.unwrap()).to_string());
                _response_lines.push(_server_functions);
                _response_lines.push(_server_id);
                _response_lines.push(rtsp_date_time());
            }
            Some(RtspCommand::Describe) => {
                let _sdp = "v=0\r\ns=Rust RTSP server\r\nt=0 0\r\nm=video 0 RTP/AVP 96\r\na=rtpmap:96 H264/90000".to_owned();
                let _sdp_byte_count: i32 = _sdp.len() as i32;
                _response_lines.push(_header_ok);
                _response_lines.push("CSeq: ".to_owned() + &(self.cseq.unwrap()).to_string());
                _response_lines.push("Content-Type: ".to_owned() + "application/sdp");
                _response_lines
                    .push("Content-Base: ".to_owned() + &self.uri.as_ref().unwrap().to_string());
                _response_lines.push("Server: ".to_owned() + "Rust RTSP Server");
                _response_lines.push(rtsp_date_time());
                _response_lines
                    .push("Content-Length: ".to_owned() + &(_sdp_byte_count).to_string() + "\r\n");
                _response_lines.push(_sdp);
            }
            Some(RtspCommand::Play) => (),
            Some(RtspCommand::Setup) => (),
            Some(RtspCommand::Teardown) => (),
            _ => (),
        };
        return Some(_response_lines.join("\r\n") + "\r\n\r\n");
    }

    fn parse_as_rtsp(_raw: String) -> Option<RtspRequest> {
        let raw_split = _raw.split("\r\n");
        let lines: Vec<&str> = raw_split.collect();
        let header: Vec<&str> = lines[0].split(" ").collect();
        let _cmd = match header[0] {
            "OPTIONS" => Some(RtspCommand::Options),
            "DESCRIBE" => Some(RtspCommand::Describe),
            "PLAY" => Some(RtspCommand::Play),
            "SETUP" => Some(RtspCommand::Setup),
            "TEARDOWN" => Some(RtspCommand::Teardown),
            _ => {
                println!("Unknown Message Type");
                None
            }
        };

        let mut _cseq: i32 = 0;
        for line in lines {
            let key_val: Vec<&str> = line.split(": ").collect();
            match key_val[0] {
                "CSeq" => {
                    _cseq = key_val[1].parse::<i32>().unwrap();
                }
                "Transport" => println!("Transport {}", key_val[1]),
                "Session" => println!("Session {}", key_val[1]),
                _ => (),
            };
        }

        return Some(RtspRequest {
            command: _cmd,
            uri: Some(header[1].to_owned()),
            rtsp_spec_version: Some(header[2].to_owned()),
            cseq: Some(_cseq),
            user_agent: None,
            session: None,
            transport: None,
        });
    }
}
