pub struct RtspRequest {
    command: &'static str,
    uri: &'static str,
    rtsp_spec_version: &'static str,
    cseq: i32,
    user_agent: &'static str,
}

pub fn parse(_raw: &str) -> RtspRequest {
    let raw_split = _raw.split("\r\n");
    let lines: Vec<&str> = raw_split.collect();
    for l in &lines[0..1] {
        let line_split: Vec<&str> = l.split(" ").collect();
        match line_split[0] {
            "OPTIONS" => println!("OPTIONS COMMAND"),
            "DESCRIBE" => println!("Not Implemented"),
            "PLAY" => println!("Not Implemented"),
            "SETUP" => println!("Not Implemented"),
            "TEARDOWN" => println!("Not Implemented"),
            _ => println!("Unknown Message Type"),
        }
    }

    return RtspRequest {
        command: "",
        uri: "",
        rtsp_spec_version: "",
        cseq: 1,
        user_agent: "",
    };
}
