extern crate rosc;

use rosc::OscMessage;
use rosc::OscPacket;
use rosc::OscType;
use rosc::OscColor;


use std::net::{UdpSocket, ToSocketAddrs};

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

    pub fn send(&self, message: &str) -> Result<(), String> {
        
        // let bytes = message.as_bytes();
        
        // let server_address = self.server_address.to_socket_addrs().map_err(|e| format!("{}", e))?.next().ok_or("No address found")?;
        // println!("Sending OSC message to {}: {}", server_address, std::str::from_utf8(bytes).unwrap());
        // self.socket.send_to(bytes, server_address).map_err(|e| format!("{}", e))?;
        // println!("OSC message sent successfully");
        let address = "/example";
        let color = OscColor {red:255, green:0, blue:0, alpha:255};
        let args: Vec<OscType> = vec![
            OscType::Int(42),
            OscType::Float(3.14),
            OscType::String("hello".to_string()),
        ];
        // let message = OscMessage {
        //     addr: address.to_string(),
        //     args: args,
        //     // args: vec![OscColor::rgba(255, 0, 0, 255)]
        // };
        let message = OscMessage {
            addr: "/color".to_string(),
            args: vec![color],  // Add the color as an argument
        };
        let bytes = rosc::encoder::encode(&OscPacket::Message(message)).unwrap();
        self.socket.send_to(&bytes, "192.168.1.43:8000").unwrap();
        
        Ok(())
    }
}