use std::net::{TcpStream, Ipv4Addr, SocketAddr, IpAddr};
use std::sync::Mutex;
use std::sync::Arc;
use bincode::{deserialize, serialize};
use std::collections::HashMap;
use crate::crypto::test;
use crate::structures::{Transaction, Message};

mod network;
mod frontend;
mod crypto;
mod structures;

fn handle(network: &network::Network, msg: &Message) {
    println!("handling data {:?}", msg);
}


fn main() {
    let mut network = network::new(handle);
    let rx_incoming_connections = network.setup();

    //async input handling
    let rx_input = frontend::init();

    loop {

        //input
        let line = rx_input.try_recv().unwrap_or(vec!["-1".to_string()]);
        match line[0].as_str() {
            "flood" => network.flood_transaction(&Transaction{
                from: vec![],
                to:vec![],
                amount: 0,
                signature: vec![]
            }),
            _ => ()
        }

        //network
        network.listen_connection(&rx_incoming_connections);
        network.listen_data();
    }
}


