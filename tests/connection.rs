//! Live STUN Binding Requests against the packaged servers (UDP, IPv4).

#![cfg(feature = "packaged")]

use std::{error::Error, sync::Arc};

use public_stun::{Capabilities, Capability, Server, packaged};
use stun::{
    agent::TransactionId,
    attributes::ATTR_OTHER_ADDRESS,
    client::ClientBuilder,
    message::{BINDING_REQUEST, BINDING_SUCCESS, Message},
};
use tokio::{net::UdpSocket, sync::mpsc, task};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Public servers come and go, so only require most of them to answer.
const MIN_SUCCESS_PERCENT: usize = 90;

/// Sends a Binding Request to `server` and waits for the response.
async fn send_binding_request(server: &Server) -> Result<Message> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(server.to_string()).await?;
    let mut client = ClientBuilder::new().with_conn(Arc::new(socket)).build()?;

    let mut request = Message::new();
    request.build(&[Box::<TransactionId>::default(), Box::new(BINDING_REQUEST)])?;

    let (tx, mut rx) = mpsc::unbounded_channel();
    client.send(&request, Some(Arc::new(tx))).await?;
    let event = rx.recv().await.ok_or("send failed")?;
    client.close().await?;

    Ok(event.event_body?)
}

/// Checks that `server` answers with a Binding Success Response, which for
/// NAT testing servers must also include an OTHER-ADDRESS (RFC 5780).
async fn check_server(server: &Server, nat_testing: bool) -> Result<()> {
    let response = send_binding_request(server).await?;

    if response.typ != BINDING_SUCCESS {
        return Err(format!("unexpected response {}", response.typ).into());
    }
    if nat_testing && !response.contains(ATTR_OTHER_ADDRESS) {
        return Err("missing OTHER-ADDRESS".into());
    }
    Ok(())
}

/// Checks all packaged servers with the given capabilities concurrently.
async fn check_servers(caps: impl Into<Capabilities>) {
    let caps = caps.into();
    let nat_testing = caps.contains(Capability::NatTesting);
    let servers = packaged::list_servers_with_capabilities(caps);

    // `stun::client::Client` is not `Sync`, so the checks run as local tasks.
    let results = task::LocalSet::new()
        .run_until(async {
            let tasks: Vec<_> = servers
                .iter()
                .cloned()
                .map(|server| {
                    task::spawn_local(async move { check_server(&server, nat_testing).await })
                })
                .collect();

            let mut results = Vec::with_capacity(tasks.len());
            for task in tasks {
                results.push(task.await.unwrap());
            }
            results
        })
        .await;

    let failures: Vec<_> = servers
        .iter()
        .zip(results)
        .filter_map(|(server, result)| result.err().map(|e| format!("{server}: {e}")))
        .collect();

    for failure in &failures {
        eprintln!("failed: {failure}");
    }

    let succeeded = servers.len() - failures.len();
    assert!(
        succeeded * 100 >= servers.len() * MIN_SUCCESS_PERCENT,
        "only {succeeded} of {} servers answered (need {MIN_SUCCESS_PERCENT}%)",
        servers.len(),
    );
}

#[tokio::test]
async fn all() {
    check_servers(Capabilities::empty()).await;
}

#[tokio::test]
async fn nat_testing() {
    check_servers(Capability::NatTesting).await;
}
