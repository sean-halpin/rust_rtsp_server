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

impl RawMessage for RtspRequest {
    fn parse_as_rtsp(_raw: String) -> Option<RtspRequest> {
        let raw_split = _raw.split("\r\n");
        let lines: Vec<&str> = raw_split.collect();
        let header: Vec<&str> = lines[0].split(" ").collect();
        match header[0] {
            "OPTIONS" => {
                println!("OPTIONS COMMAND");
                let _cmd = Some(RtspCommand::Options);
                for line in lines {
                    let kv: Vec<&str> = line.split(": ").collect();
                    match kv[0] {
                        "CSeq" => println!("CSeq {}", kv[1].parse::<i32>().unwrap()),
                        "Transport" => println!("Transport {}", kv[1]),
                        "Session" => println!("Session {}", kv[1]),
                        _ => (),
                    };
                }
                return Some(RtspRequest {
                    command: None,
                    uri: None,
                    rtsp_spec_version: None,
                    cseq: None,
                    user_agent: None,
                    session: None,
                    transport: None,
                });
            }
            "DESCRIBE" => println!("Not Implemented"),
            "PLAY" => println!("Not Implemented"),
            "SETUP" => println!("Not Implemented"),
            "TEARDOWN" => println!("Not Implemented"),
            _ => {
                println!("Unknown Message Type");
                return None;
            }
        }

        return None;
    }
}
