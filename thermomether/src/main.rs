use std::{net::UdpSocket, thread, time::Duration};

fn main() {
    let udp = UdpSocket::bind("127.0.0.1:9999").unwrap();
    let server = "127.0.0.1:10001";

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..120 {
                let temp = 20 + i ;
                let _ = udp.send_to(&i32::to_be_bytes(temp), server);

                thread::sleep(Duration::from_secs(10));
            }
        });
    });
}
