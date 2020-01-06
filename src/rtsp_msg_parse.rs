extern crate chrono;
use chrono::Utc;

pub enum RtspCommand {
    Options,
    Describe,
    Setup,
    Play,
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
    content_base: Option<String>,
    cseq: Option<i32>,
    session: Option<String>,
    transport: Option<String>,
}

fn rtsp_date_time() -> String {
    return "Date: ".to_owned() + &Utc::now().to_rfc2822();
}

impl RawMessage for RtspRequest {
    fn response(&self) -> Option<String> {
        let _header_ok = "RTSP/1.0 200 OK".to_owned();
        let _server_id = "Server: Rust RTSP server".to_owned();
        let mut _response_lines: Vec<String> = Vec::new();

        match self.command {
            Some(RtspCommand::Options) => {
                let _server_functions =
                    "Public: OPTIONS, DESCRIBE, PLAY, SETUP, TEARDOWN".to_owned();
                _response_lines.push(_header_ok);
                _response_lines.push("CSeq: ".to_owned() + &(self.cseq.unwrap()).to_string());
                _response_lines.push(_server_functions);
                _response_lines.push(_server_id);
                _response_lines.push(rtsp_date_time());
                _response_lines.push("\r\n".to_owned());
            }
            Some(RtspCommand::Describe) => {
                let _sdp = "v=0\r\ns=Rust RTSP server\r\nt=0 0\r\nm=video 0 RTP/AVP 96\r\na=rtpmap:96 H264/90000".to_owned();
                let _sdp_byte_count: i32 = _sdp.len() as i32;
                _response_lines.push(_header_ok);
                _response_lines.push("CSeq: ".to_owned() + &(self.cseq.unwrap()).to_string());
                _response_lines.push("Content-Type: ".to_owned() + "application/sdp");
                _response_lines.push(
                    "Content-Base: ".to_owned() + &self.content_base.as_ref().unwrap().to_string(),
                );
                _response_lines.push(_server_id);
                _response_lines.push(rtsp_date_time());
                _response_lines
                    .push("Content-Length: ".to_owned() + &(_sdp_byte_count).to_string() + "\r\n");
                _response_lines.push(_sdp);
            }
            Some(RtspCommand::Setup) => {
                let session_id = 1;
                _response_lines.push(_header_ok);
                _response_lines.push("CSeq: ".to_owned() + &(self.cseq.unwrap()).to_string());
                _response_lines.push(
                    "Transport: RTP/AVP;unicast;client_port=10500-10501;server_port=5700-5701;mode=\"PLAY\""
                        .to_owned(),
                );
                _response_lines.push(_server_id);
                _response_lines.push("Session: ".to_owned() + &session_id.to_string());
                _response_lines.push(rtsp_date_time());
                _response_lines.push("\r\n".to_owned());
            }
            Some(RtspCommand::Play) => {
                let session_id = 1;
                _response_lines.push("CSeq: ".to_owned() + &(self.cseq.unwrap()).to_string());
                _response_lines.push(
                    "RTP-Info: url=: ".to_owned()
                        + &self.content_base.as_ref().unwrap().to_string()
                        + "stream=0;seq=1;rtptime=0",
                );
                _response_lines.push("Range: npt=0-".to_owned());
                _response_lines.push(_server_id);
                _response_lines.push("Session: ".to_owned() + &session_id.to_string());
                _response_lines.push(rtsp_date_time());
                _response_lines.push("\r\n".to_owned());
            }

            Some(RtspCommand::Teardown) => (),
            _ => (),
        };
        return Some(_response_lines.join("\r\n"));
    }

    fn parse_as_rtsp(_raw: String) -> Option<RtspRequest> {
        let raw_split = _raw.split("\r\n");
        let lines: Vec<&str> = raw_split.collect();
        let header: Vec<&str> = lines[0].split(" ").collect();
        let _cmd = match header[0] {
            "OPTIONS" => Some(RtspCommand::Options),
            "DESCRIBE" => Some(RtspCommand::Describe),
            "SETUP" => Some(RtspCommand::Setup),
            "PLAY" => Some(RtspCommand::Play),
            "TEARDOWN" => Some(RtspCommand::Teardown),
            _ => {
                println!("Unknown Message Type");
                None
            }
        };

        let mut _cseq: i32 = 0;
        let mut transport = String::new();
        let mut session = String::new();
        for line in lines {
            let key_val: Vec<&str> = line.split(": ").collect();
            match key_val[0] {
                "CSeq" => {
                    _cseq = key_val[1].parse::<i32>().unwrap();
                }
                "Transport" => transport = key_val[1].to_owned(),
                "Session" => session = key_val[1].to_owned(),
                _ => (),
            };
        }

        return Some(RtspRequest {
            command: _cmd,
            content_base: Some(header[1].to_owned()),
            cseq: Some(_cseq),
            session: Some(session),
            transport: Some(transport),
        });
    }
}
