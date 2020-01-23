use crate::rtsp_msg_handler::RtspMessage;
use std::net::TcpListener;

#[derive(Clone)]
pub struct RtspSession {
    pub client_rtp: String,
    pub client_rtcp: String,
    pub server_rtcp: String,
}

pub trait ClientPorts {
    fn record_client_ports(msg: RtspMessage) -> RtspSession;
}

fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

impl ClientPorts for RtspSession {
    fn record_client_ports(msg: RtspMessage) -> RtspSession {
        let server_rtcp_port = (12000..50000).find(|port| port_is_available(*port));
        return RtspSession {
            client_rtp: msg.client_rtp.unwrap_or_default(),
            client_rtcp: msg.client_rtcp.unwrap_or_default(),
            server_rtcp: server_rtcp_port.unwrap_or_default().to_string(),
        };
    }
}
