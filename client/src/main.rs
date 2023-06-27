use std::{io, sync::Arc};

use tdtp::client;

use tokio::net::UdpSocket;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = client::Client {
        udp: UdpSocket::bind("127.0.0.1:0").await.unwrap(),
    };

    let client = Arc::new(client);
    let arc_client = client.clone();

    let timeout = tokio::time::Duration::from_secs(2);

    let mut buf = String::new();
    loop {
        println!("----------\n1) Get current temerature\n_) Exit");
        buf.clear();
        let _ = io::stdin().read_line(&mut buf);
        let input = buf.trim().parse::<i32>().unwrap();
        println!("Chosed: {}", &input);
        match input {
            1 => {
                let mut buf = [0u8; 4];

                let result = tokio::time::timeout(timeout, 
                    async {
                        match arc_client.send_command(client::ClientCommand::GetTemp, "127.0.0.1:10002").await {
                            Err(e) => {println!("Send err: {e}")},
                            Ok(_) => {},
                        };

                        match arc_client.udp.recv(&mut buf).await {
                            Err(e) => {println!("Send err: {e}")},
                            Ok(_) => {
                                let temp = i32::from_be_bytes(buf);
                                println!("Current temperature: {}", temp);
                            },
                        };
                    }
                ).await;

                match result {
                    Err(e) => {println!("Timeout error: {e}")},
                    Ok(_) => {},
                }
            },
            _ => {
                break;
            }
        }
    }
}
