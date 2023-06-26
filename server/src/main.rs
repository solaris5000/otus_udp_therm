use std::{sync::{Arc, RwLock}, rc::Rc};
use tdtp::server::{*, self};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client_server = ThermometerServer::start_incoming().await;
    let term_server = ThermometerServer::start_internal().await;
    
    let temp_data = Arc::new(RwLock::new(10));

    let arc_term = Arc::new(term_server);
    let arc_client = Arc::new(client_server);

    let term_server = arc_term.clone();
    let server_data = temp_data.clone();
    tokio::spawn( async move {
        ThermometerServer::listen_term(&term_server.udp, server_data).await;
    });

    loop {
        let client_server = arc_client.clone();

        let client_data = temp_data.clone();
        ThermometerServer::listen_client(&client_server.udp, client_data).await;

        println!("test");
    }
    

    println!("servers closed");
}
