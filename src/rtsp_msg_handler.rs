extern crate chrono;
use chrono::Utc;

#[derive(Clone, Debug)]
pub enum RtspCommand {
    Options,
    Describe,
    Setup,
    Play,
    Teardown,
}

pub trait RtspParsable {
    fn parse_as_rtsp(raw: String) -> Option<Self>
    where
        Self: std::marker::Sized;
}

pub trait RtspResponse {
    fn response(&self) -> Option<String>;
}

#[derive(Clone, Debug)]
pub struct RtspMessage {
    pub command: Option<RtspCommand>,
    pub content_base: Option<String>,
    pub cseq: Option<String>,
    pub session: Option<String>,
    pub client_rtp: Option<String>,
    pub client_rtcp: Option<String>,
}

fn rtsp_date_time() -> String {
    return "Date: ".to_owned() + &Utc::now().to_rfc2822();
}

impl RtspResponse for RtspMessage {
    fn response(&self) -> Option<String> {
        let _header_ok = "RTSP/1.0 200 OK".to_owned();
        let _server_id = "Server: Rust RTSP server".to_owned();
        let mut _response_lines: Vec<String> = Vec::new();

        match self.command {
            Some(RtspCommand::Options) => {
                let _server_functions =
                    "Public: OPTIONS, DESCRIBE, PLAY, SETUP, TEARDOWN".to_owned();
                _response_lines.push(_header_ok);
                _response_lines
                    .push("CSeq: ".to_owned() + &(self.cseq.as_ref().unwrap()).to_string());
                _response_lines.push(_server_functions);
                _response_lines.push(_server_id);
                _response_lines.push(rtsp_date_time());
                _response_lines.push("\r\n".to_owned());
            }
            Some(RtspCommand::Describe) => {
                let _sdp = "v=0\r\ns=Rust RTSP server\r\nt=0 0\r\nm=video 0 RTP/AVP 96\r\na=rtpmap:96 H264/90000".to_owned();
                let _sdp_byte_count: i32 = _sdp.len() as i32;
                _response_lines.push(_header_ok);
                _response_lines
                    .push("CSeq: ".to_owned() + &(self.cseq.as_ref().unwrap()).to_string());
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
                _response_lines
                    .push("CSeq: ".to_owned() + &(self.cseq.as_ref().unwrap()).to_string());
                _response_lines.push(
                    format!("Transport: RTP/AVP;unicast;client_port={}-{};server_port=5700-5701;mode=\"PLAY\"",
                    &self.client_rtp.as_ref().unwrap(),
                    &self.client_rtcp.as_ref().unwrap()),
                );
                _response_lines.push(_server_id);
                _response_lines.push("Session: ".to_owned() + &session_id.to_string());
                _response_lines.push(rtsp_date_time());
                _response_lines.push("\r\n".to_owned());
            }
            Some(RtspCommand::Play) => {
                let session_id = 1;
                _response_lines.push(_header_ok);
                _response_lines
                    .push("CSeq: ".to_owned() + &(self.cseq.as_ref().unwrap()).to_string());
                _response_lines.push(
                    "RTP-Info: url=:".to_owned()
                        + &self.content_base.as_ref().unwrap().to_string()
                        + "/stream=0;seq=1;rtptime=0",
                );
                _response_lines.push("Range: npt=0-".to_owned());
                _response_lines.push(_server_id);
                _response_lines.push("Session: ".to_owned() + &session_id.to_string());
                _response_lines.push(rtsp_date_time());
                _response_lines.push("\r\n".to_owned());
            }
            Some(RtspCommand::Teardown) => (),
            _ => return None,
        };
        return Some(_response_lines.join("\r\n"));
    }
}

impl RtspParsable for RtspMessage {
    fn parse_as_rtsp(_raw: String) -> Option<RtspMessage> {
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

        let _content_base = if header[1].contains("rtsp://") {
            Some(header[1].to_owned())
        } else {
            None
        };

        let mut _cseq: Option<String> = None;
        let mut _rtp: Option<String> = None;
        let mut _rtcp: Option<String> = None;
        let mut _session: Option<String> = None;
        for line in lines {
            let key_val: Vec<&str> = line.split(": ").collect();
            match key_val[0] {
                "CSeq" => {
                    _cseq = Some(key_val[1].to_owned());
                }
                "Transport" => {
                    let _transport = key_val[1].to_owned();
                    let _transport_split: Vec<&str> = key_val[1].split(";").collect();
                    for _item in _transport_split {
                        if _item.contains("client_port") {
                            let _key_val: Vec<&str> = _item.split("=").collect();
                            let transport: Vec<&str> = _key_val[1].split("-").collect();
                            _rtp = Some(transport[0].to_owned());
                            _rtcp = Some(transport[1].to_owned());
                        }
                    }
                }
                "Session" => _session = Some(key_val[1].to_owned()),
                _ => (),
            };
        }

        return Some(RtspMessage {
            command: _cmd,
            content_base: _content_base,
            cseq: _cseq,
            session: _session,
            client_rtp: _rtp,
            client_rtcp: _rtcp,
        });
    }
}
