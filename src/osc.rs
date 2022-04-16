use std::{convert::From};
use async_std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use rosc::{OscMessage, OscType, OscPacket};

pub enum Msg {
    MoveForward(bool),
    Run(bool),
}

fn input_msg(addr: String, is_active: bool) -> OscMessage {
    OscMessage {
        addr,
        args: vec![OscType::Int(if is_active { 1 } else { 0 })]
    }
}

impl From<Msg> for OscMessage {
    fn from(item: Msg) -> Self {
        match item {
            Msg::MoveForward(is_active) =>
                input_msg("/input/MoveForward".to_owned(), is_active),
            Msg::Run(is_active) =>
                input_msg("/input/Run".to_owned(), is_active),
        }
    }
}

pub struct Client {
    udp_client: UdpSocket,
    to_addr: SocketAddrV4,
}

impl Client {
    pub async fn new() -> Self {
        Self {
            udp_client: UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9001)).await.unwrap(),
            to_addr: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9000),
        }
    }

    pub async fn send_msg(&self, msg: Msg) -> Result<(), Box<dyn std::error::Error>> {
        let osc_msg = msg.into();
        let msg_buf = rosc::encoder::encode(&OscPacket::Message(osc_msg))?;

        self.udp_client.send_to(&msg_buf, self.to_addr).await?;

        Ok(())
    }
}

