use std::net::{UdpSocket};

pub struct ThermometerServer {
    udp : UdpSocket,
}

impl ThermometerServer {
    pub fn start_internal() -> ThermometerServer{
        ThermometerServer {udp : UdpSocket::bind("127.0.0.1:10001").unwrap()}
    }

    pub fn start_incoming() -> ThermometerServer{
        ThermometerServer {udp : UdpSocket::bind("127.0.0.1:10002").unwrap()}
    }

    pub fn listen(&self) {
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

        println!("Listener closed");
    }

    pub fn renew() {

    }
}
