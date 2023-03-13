mod client;
mod player;
mod stream;

use client::handle_client;
use client::ClientStatus;
use client::{Client, ClientPool, ClientRef};
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

async fn broadcast_main(client_pool: ClientPool) -> () {
    loop {
        for _ in client_pool.lock().await.iter() {
            // println!("CLIENT!");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "localhost:8080".to_string());

    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on: {}", addr);

    let client_pool: ClientPool = Arc::new(Mutex::new(Vec::new()));

    tokio::spawn(broadcast_main(client_pool.clone()));

    loop {
        let (socket, _) = listener.accept().await?;

        let client: ClientRef = Arc::new(Mutex::new(Client {
            tcp_socket: socket,
            ip_address: "".to_owned(),
            rdp_port: 0u16,
            status: ClientStatus::Idle,
        }));

        client_pool.lock().await.push(client.clone());
        tokio::spawn(handle_client(client));
    }
}
