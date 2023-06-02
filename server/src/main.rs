use std::net::{ToSocketAddrs, UdpSocket};
use std::sync::{Arc, RwLock};
use std::thread;
use tdtp::server::*;

fn main() {
    let client_server = ThermometerServer::start_incoming();
    let term_server = ThermometerServer::start_internal();

    let temp_data = Arc::new(RwLock::new(0));

    thread::scope(|s| {
        s.spawn(|| {
            client_server.listen_client(temp_data.clone());
        });

        s.spawn(|| {
            term_server.listen_term(temp_data.clone());
        });

        println!("servers started");
    });

    println!("servers closed");
}
