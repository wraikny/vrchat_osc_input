use std::{convert::From};
use async_std::{channel::Receiver, net::{Ipv4Addr, SocketAddrV4, UdpSocket}};

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
    receiver: Receiver<Msg>,
    udp_client: UdpSocket,
    to_addr: SocketAddrV4,
}

impl Client {
    pub async fn new(receiver: Receiver<Msg>) -> Self {
        Self {
            receiver,
            udp_client: UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9001)).await.unwrap(),
            to_addr: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9000),
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            if let Ok(msg) = self.receiver.recv().await {
                let osc_msg = msg.into();
                let msg_buf = rosc::encoder::encode(&OscPacket::Message(osc_msg))?;

                
                self.udp_client.send_to(&msg_buf, self.to_addr).await?;
            }
        }
    }
}

