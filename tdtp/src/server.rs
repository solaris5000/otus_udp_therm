use tokio::net::UdpSocket;
use std::sync::{Arc, RwLock};
pub struct ThermometerServer {
    pub udp: UdpSocket,
}

impl ThermometerServer {
    pub async fn start_internal() -> ThermometerServer {
        ThermometerServer {
            udp: UdpSocket::bind("127.0.0.1:10001").await.unwrap(),
        }
    }

    pub async fn start_incoming() -> ThermometerServer {
        ThermometerServer {
            udp: UdpSocket::bind("127.0.0.1:10002").await.unwrap(),
        }
    }

    // сделать 2 разных прослушивателя, 1 для клиента 1 для термометра
    pub async fn listen_term(connection : &UdpSocket, temp_data: Arc<RwLock<i32>>) {
        let mut buf = [0u8; 4];

        loop {
            let rt = connection.recv_from(&mut buf).await;
            match rt {
                Err(e) => {
                    println!("Somthing went wrong\n{:?}", e);
                }
                Ok(r) => {
                    let size = r.0;
                    let sender = r.1;
                    let data = i32::from_be_bytes(buf);
                    println!(
                        "Recived {} bytes from {}\nTemperature: {}",
                        &size, &sender, &data,
                    );
                    let mut temp = temp_data.write().unwrap();
                    *temp = data;
                }
            }
        }
    }
    pub async fn listen_client(connection : &UdpSocket, temp_data: Arc<RwLock<i32>>) {
        let mut buf = [0; 4];
            let rt = connection.recv_from(&mut buf).await;
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
                        return;
                    }

                    match msg[..].trim() {
                        "TEMP" => {
                            let data = temp_data.read().unwrap();
                            println!("Sending recponce: {:?}", &data.to_be_bytes());
                            let _ = connection.send_to(&data.to_be_bytes(), &sender).await;
                        }
                        _ => {
                            println!("Sending recponce: {:?}", "WCMD");
                            let _ = connection.send_to(b"WCMD", &sender).await;
                        }
                    }
                }
        }

        println!("Listener closed");
    }
}
