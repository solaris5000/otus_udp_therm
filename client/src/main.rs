use::std::net::{UdpSocket, ToSocketAddrs};

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8095").unwrap();
    let bytes = b"test";
    let bytes_end = b"end";
    let bytesa = b"test string";
    let _ = socket.send_to(bytes, "127.0.0.1:10001");
    let _ = socket.send_to(bytes_end, "127.0.0.1:10001");
    let _ = socket.send_to(bytesa, "127.0.0.1:10002");
    let _ = socket.send_to(bytes_end, "127.0.0.1:10002");
}
