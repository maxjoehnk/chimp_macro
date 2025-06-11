use std::net::{Ipv4Addr, UdpSocket};
use std::time::Duration;
use rosc::{OscMessage, OscPacket, OscType};
use crate::chimp::ChimpConnection;

pub struct OscConnection {
    pub(crate) socket: UdpSocket,
    addr: Ipv4Addr,
}

impl OscConnection {
    pub fn new(addr: Ipv4Addr) -> Self {
        let socket = UdpSocket::bind("0.0.0.0:9000").unwrap();
        socket.set_nonblocking(true).unwrap();
        socket.connect((addr, 8000)).unwrap();

        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();

        Self { socket, addr }
    }

    pub fn ping(&self) {
        let ping_packet = OscPacket::Message(OscMessage {
            addr: "/ping".into(),
            args: vec![],
        });

        let ping_packet = rosc::encoder::encode(&ping_packet).unwrap();

        self.socket.send(&ping_packet).unwrap();
    }
}

impl ChimpConnection for OscConnection {
    fn send_bool(&self, addr: String, value: bool) {
        let packet = OscPacket::Message(OscMessage {
            addr,
            args: vec![OscType::Float(value.into())],
        });

        let bytes = rosc::encoder::encode(&packet).unwrap();

        self.socket.send(&bytes).unwrap();
    }

    fn send_msg(&self, addr: String) {
        let packet = OscPacket::Message(OscMessage {
            addr,
            args: vec![],
        });

        let bytes = rosc::encoder::encode(&packet).unwrap();

        self.socket.send(&bytes).unwrap();
    }
}
