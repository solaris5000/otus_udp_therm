use std::{io, net::UdpSocket, thread};

use tdtp::client;

fn main() {
    let client = client::Client {
        udp: UdpSocket::bind("127.0.0.1:0").unwrap(),
    };

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
                thread::scope(|s|{
                 s.spawn(||{
                client.send_command(client::ClientCommand::GetTemp, "127.0.0.1:10002");
                match client.udp.recv(&mut buf) {
                    Err(e) => {
                        println!("Something went wrong: {:?}", e);
                    }
                    Ok(_) => {
                        let temp = i32::from_be_bytes(buf);
                        println!("Current temperature: {}", temp);
                    }
                }

                  });
                  });
            }
            _ => {
                break;
            }
        }
    }
}
