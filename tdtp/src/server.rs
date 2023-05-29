use std::net::{UdpSocket, ToSocketAddrs};

pub struct ThermServer {
    pub tcp: TcpListener,
}

impl ThermServer {
    pub fn start_server<Addrs>(addr: Addrs) -> SocketServer
    where
        Addrs: ToSocketAddrs,
    {
        let temp = TcpListener::bind(addr);
        SocketServer { tcp: temp.unwrap() }
    }
}

impl std::fmt::Debug for SocketServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "this is a socket")
    }
}
