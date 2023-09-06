#![allow(unreachable_code)]
use std::{error::Error, net::SocketAddr};

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Running udp echo!");
    let local_address: SocketAddr = "127.0.0.1:10200".parse()?;

    let socket = UdpSocket::bind(local_address).await?;

    let mut buffer = vec![0; 1500];
    loop {
        let (amount, from) = socket.recv_from(&mut buffer).await?;
        println!(
            "Received {:?} {amount:?} bytes from {from:?}.",
            &buffer[..amount]
        );

        let sent = socket
            .send_to(&["Echo:".as_bytes(), &buffer[..amount]].concat(), from)
            .await?;
        println!("Sending reply {sent:?} bytes to {from:?}.");
    }

    Ok(())
}
