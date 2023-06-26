use std::{sync::{Arc, RwLock}, rc::Rc, thread, time::Duration};
use tdtp::server::{*, self};
use tokio::sync::Mutex;
use tokio::time;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client_server = ThermometerServer::start_incoming().await;
    let term_server = ThermometerServer::start_internal().await;

    let client_server = Arc::new(client_server);
    
    let temp_data = Arc::new(Mutex::new(10));
    let therm_data = temp_data.clone();

    //let arc_term = Arc::new(term_server);
    //let arc_client = Arc::new(client_server);

    tokio::spawn( async move {
        ThermometerServer::listen_term(&term_server.udp, therm_data.clone()).await;
    });

    loop {

        let client_data = temp_data.clone();
        let udp = client_server.clone();

       // tokio::spawn( async move {
            let result = tokio::time::timeout(Duration::from_secs(1), 
       (async {ThermometerServer::listen_client(udp, client_data).await})).await;
        //});
        //println!("{:?}", result);
        println!("test");
        //thread::sleep(Duration::from_secs(1));
    }
    

    println!("servers closed");
}
