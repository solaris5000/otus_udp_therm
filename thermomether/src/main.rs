use std::{net::{UdpSocket, ToSocketAddrs}, sync::Arc};
use std::thread;
use tdtp::server::*;



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