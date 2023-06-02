use std::io::Read;
use std::net::{ToSocketAddrs, UdpSocket};

pub enum ClientCommand {
    GetTemp,
}

pub struct Client {
    pub udp: UdpSocket,
}

impl Client {
    pub fn send_command<ADR>(&self, cmd: ClientCommand, target: ADR)
    where
        ADR: ToSocketAddrs,
    {
        match cmd {
            ClientCommand::GetTemp => {
                let _ = self.udp.send_to(b"TEMP", target);
            }
        }
    }
}
