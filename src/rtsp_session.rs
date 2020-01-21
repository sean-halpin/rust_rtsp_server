pub struct RtspSession {
    pub client_rtp: Option<String>,
    pub client_rtcp: Option<String>,
}

pub trait ClientPorts {
    fn record(rtp: String, rtcp: String) -> RtspSession;
}

impl ClientPorts for RtspSession {
    fn record(rtp: String, rtcp: String) -> RtspSession {
        return RtspSession {
            client_rtp: Some(rtp),
            client_rtcp: Some(rtcp),
        };
    }
}
