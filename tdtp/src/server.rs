use std::net::UdpSocket;
use std::sync::{Arc, RwLock};
pub struct ThermometerServer {
    udp: UdpSocket,
}

impl ThermometerServer {
    pub fn start_internal() -> ThermometerServer {
        ThermometerServer {
            udp: UdpSocket::bind("127.0.0.1:10001").unwrap(),
        }
    }

    pub fn start_incoming() -> ThermometerServer {
        ThermometerServer {
            udp: UdpSocket::bind("127.0.0.1:10002").unwrap(),
        }
    }

    // сделать 2 разных прослушивателя, 1 для клиента 1 для термометра
    pub fn listen_term(&self, temp_data: Arc<RwLock<f32>>) {
        let mut buf = [0u8; 4];

        loop {
            let rt = self.udp.recv_from(&mut buf);
            match rt {
                Err(e) => {
                    println!("Somthing went wrong\n{:?}", e);
                }
                Ok(r) => {
                    let size = r.0;
                    let sender = r.1;
                    println!(
                        "Recived {} bytes from {}\nTemperature: {}",
                        &size,
                        &sender,
                        i32::from_be_bytes(buf)
                    );
                    if &size == &(3 as usize) {
                        break;
                    }
                }
            }
        }
    }
    pub fn listen_client(&self, temp_data: Arc<RwLock<f32>>) {
        let mut buf = [0; 32];

        loop {
            let rt = self.udp.recv_from(&mut buf);
            match rt {
                Err(e) => {
                    println!("Somthing went wrong\n{:?}", e);
                }
                Ok(r) => {
                    let size = r.0;
                    let sender = r.1;
                    let vec_buf = buf.to_vec();
                    let msg = String::from_utf8(vec_buf)
                        .unwrap_or("Bytes to UTF-8 convert ERR".to_string());
                    println!(
                        "Recived {} bytes from {}\nData {}",
                        &size,
                        &sender,
                        &msg[..].trim_end()
                    );
                    if &size == &(3 as usize) {
                        break;
                    }
                }
            }
        }

        println!("Listener closed");
    }

    pub fn renew(term_data: Arc<RwLock<f32>>) {}
}
