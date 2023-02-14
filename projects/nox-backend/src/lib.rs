#![feature(async_closure)]

use std::net::SocketAddr;

use dashmap::DashMap;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message;

use nox_types::NoxResult;

pub type Tx = UnboundedSender<Message>;

pub async fn handle_connection(peer_map: DashMap<SocketAddr, Tx>, raw_stream: TcpStream, addr: SocketAddr) -> NoxResult {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = accept_async(raw_stream).await?;
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    peer_map.insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!("Received a message from {}: {}", addr, msg.to_text().unwrap());

        // We want to broadcast the message to everyone except ourselves.
        let broadcast_recipients =
            peer_map.iter().filter(|peer_addr| peer_addr.key() != &addr).map(|ws_sink| ws_sink.value().clone());

        for recp in broadcast_recipients {
            recp.unbounded_send(msg.clone()).unwrap();
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.remove(&addr);
    Ok(())
}

