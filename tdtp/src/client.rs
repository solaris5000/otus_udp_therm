use std::io::Read;
use std::net::{ToSocketAddrs, UdpSocket};

pub enum ClientCommand {
    GetTemp,
}

pub fn read_responce<Reader: Read>(mut reader: Reader) -> String {
    "recponce".to_string()
}

pub fn send_command<ADR>(cmd: ClientCommand, target: ADR)
where
    ADR: ToSocketAddrs,
{
    let udp = UdpSocket::bind("127.0.0.1:9992").unwrap();
    match cmd {
        ClientCommand::GetTemp => {
            let _ = udp.send_to(b"TEMP", target);
        }
    }
}
