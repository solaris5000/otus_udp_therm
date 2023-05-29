use std::{net::{UdpSocket, ToSocketAddrs}, sync::Arc};
use std::thread;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Thermometer {
    pub name: String,
    pub room_name: String,
    pub temp: f32,
}

impl std::fmt::Display for Thermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Thermometer: {} \nLocation: {}",
            self.name, self.room_name
        )
    }
}

impl Thermometer {
    pub fn new(name: &str) -> Self {
        Thermometer {
            name: (name.to_owned()),
            room_name: ("Unknown".to_owned()),
            temp: (0.0),
        }
    }

    pub fn _scan_temp(&mut self) {
        todo!();
    }

    pub fn _get_temp(&mut self) {
        self._scan_temp();
        todo!();
    }
}

struct ThermometerServer {
    udp : UdpSocket,
}

impl ThermometerServer {
    fn start_internal() -> ThermometerServer{
        ThermometerServer {udp : UdpSocket::bind("127.0.0.1:10001").unwrap()}
    }

    fn start_incoming() -> ThermometerServer{
        ThermometerServer {udp : UdpSocket::bind("127.0.0.1:10002").unwrap()}
    }

    fn listen(&self) {
        let mut buf = [0; 32];

        loop {
            let rt = self.udp.recv_from(&mut buf);
            match rt {
                Err(e) => { 
                    println!("Somthing went wrong\n{:?}", e);
                },
                Ok(r) => { 
                    let size = r.0; 
                    let sender = r.1;

                    println!("Recived {} bytes from {}\nData {:?}", &size, &sender, &buf);
                    if &size == &(3 as usize) {
                        break;
                    }
                }

                
            }
        };
    }

    fn renew() {

    }
}


fn main() {
    let testa = ThermometerServer::start_incoming();
    let testb = ThermometerServer::start_internal();

    thread::scope(|s| {
        s.spawn(||{
            testa.listen();
        });

        s.spawn(||{
            testb.listen();
        });

        println!("servers started");
    });

    println!("servers closed");
}