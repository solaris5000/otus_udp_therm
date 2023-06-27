use std::{sync::Arc, time::Duration};
use tdtp::server::*;
use tokio::sync::Mutex;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Сокет, принимающий пользовательский ввод с клиента
    let client_server = ThermometerServer::start_incoming().await;

    //сокет, принимающий датаграммы от термометра
    let term_server = ThermometerServer::start_internal().await;

    let client_server = Arc::new(client_server);

    let temp_data = Arc::new(Mutex::new(0));
    let therm_data = temp_data.clone();

    tokio::spawn(async move {
        ThermometerServer::listen_term(&term_server.udp, therm_data.clone()).await;
    });

    loop {
        let client_data = temp_data.clone();
        let udp = client_server.clone();

        let _ = tokio::time::timeout(Duration::from_secs(1), async {
            // раскоментируй строку ниже, чтобы получить ошибку таймаута у клиента ;)
            // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            ThermometerServer::listen_client(udp, client_data).await
        })
        .await;
    }

    //println!("servers closed");
}
