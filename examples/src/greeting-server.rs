extern crate tcp_channel;
extern crate serde;
#[macro_use] extern crate serde_derive;

use std::net::TcpListener;

mod common;
use common::{ClientToServer, ServerToClient};

use tcp_channel::{Receiver, ChannelRecv, Sender, ChannelSend};

fn main() {
    let address = std::env::args().nth(1).unwrap();
    let listener = TcpListener::bind(address).unwrap();

    while let Ok((stream, client_address)) = listener.accept() {
        println!("INFO: Started connection with {}", client_address);
        let mut receiver = Receiver::<ClientToServer>::new(stream.try_clone().unwrap());
        let mut sender = Sender::new(stream);

        while let Ok(message) = receiver.recv() {
            match message {
                ClientToServer::Say(_) => sender.send(&ServerToClient::Answer("Hi".into())).unwrap(),
                ClientToServer::Leave => sender.send(&ServerToClient::Answer("Goodbye".into())).unwrap(),
            }
        }
    }
    println!("INFO: Stopped server");
}
