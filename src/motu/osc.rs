extern crate rosc;

// use rosc::OscMessage;
use rosc::OscPacket;
// use rosc::OscType;
// use rosc::OscColor;


// use std::net::{UdpSocket, ToSocketAddrs};
use std::net::UdpSocket;

pub struct OscClient {
    socket: UdpSocket,
    server_address: String,
}

impl OscClient {
    pub fn new(server_address: &str) -> Result<Self, String> {
        let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| format!("{}", e))?;
        let server_address = server_address.to_string();
        Ok(Self { socket, server_address })
    }

    
    pub fn send(&self, packet: OscPacket) -> Result<(), String> {
        let bytes = rosc::encoder::encode(&packet).map_err(|e| format!("{}", e))?;
        self.socket
            .send_to(&bytes, &self.server_address)
            .map_err(|e| format!("{}", e))?;
        Ok(())
    }
}