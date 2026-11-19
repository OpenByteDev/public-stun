//! Prints this machine's public IP address and port as seen by a public STUN server.

use std::{error::Error, net::SocketAddr, sync::Arc};

use public_stun::Server;
use stun::{
    addr::MappedAddress,
    agent::TransactionId,
    client::ClientBuilder,
    message::{BINDING_REQUEST, Getter, Message},
    xoraddr::XorMappedAddress,
};
use tokio::{net::UdpSocket, sync::mpsc};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Asks `server` which address our request came from.
async fn get_public_address(server: &Server) -> Result<SocketAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(server.to_string()).await?;
    let mut client = ClientBuilder::new().with_conn(Arc::new(socket)).build()?;

    let mut request = Message::new();
    request.build(&[Box::<TransactionId>::default(), Box::new(BINDING_REQUEST)])?;

    let (tx, mut rx) = mpsc::unbounded_channel();
    client.send(&request, Some(Arc::new(tx))).await?;
    let event = rx.recv().await.ok_or("send failed")?;
    client.close().await?;
    let response = event.event_body?;

    // Modern servers send XOR-MAPPED-ADDRESS, older ones only MAPPED-ADDRESS.
    let mut xor = XorMappedAddress::default();
    if xor.get_from(&response).is_ok() {
        return Ok(SocketAddr::new(xor.ip, xor.port));
    }
    let mut mapped = MappedAddress::default();
    mapped.get_from(&response)?;
    Ok(SocketAddr::new(mapped.ip, mapped.port))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Public servers come and go, so try them in order until one answers.
    for server in public_stun::packaged::list_servers() {
        match get_public_address(&server).await {
            Ok(address) => {
                println!("Public address: {address} (via {server})");
                return Ok(());
            }
            Err(e) => eprintln!("{server}: {e}"),
        }
    }
    Err("no STUN server answered".into())
}
