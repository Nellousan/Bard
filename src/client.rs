use rtsp_types::{Method, Request};
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub enum ClientStatus {
    Idle,
    Setup,
    Paused,
    Playing,
}

pub struct Client {
    pub tcp_socket: TcpStream,
    pub ip_address: String,
    pub rdp_port: u16,
    pub status: ClientStatus,
}

pub type ClientRef = Arc<Mutex<Client>>;
pub type ClientPool = Arc<Mutex<Vec<ClientRef>>>;

pub async fn handle_client(client: ClientRef) -> () {
    let mut buf = vec![0; 1024];

    loop {
        let n = client
            .lock()
            .await
            .tcp_socket
            .read(&mut buf)
            .await
            .expect("failed to read on socket.");

        println!("read:\n{}", String::from_utf8_lossy(buf.as_slice()));

        if n == 0 {
            return;
        }

        handle_message(client.clone(), &buf).await;
    }
}

pub async fn handle_message(client: ClientRef, data: &Vec<u8>) -> () {
    let (message, _): (rtsp_types::Message<Vec<u8>>, _) =
        rtsp_types::Message::parse(&data).expect("Failed to parse rtsp");

    if let rtsp_types::Message::Request(request) = message {
        println!("message type: {:?}", request);
        match request.method() {
            Method::Setup => handle_setup(client, request).await,
            _ => println!("unhandled request method: {:?}", request.method()),
        }
    }
}

pub async fn handle_setup(client: ClientRef, request: Request<Vec<u8>>) -> () {
    println!("got setup")
}
