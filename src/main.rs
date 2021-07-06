use std::collections::HashMap;
use crate::crypto::{gen, key_to_string, string_to_key};
use crate::structures::{Ledger};

mod network;
mod frontend;
mod crypto;
mod structures;
mod blockchain;

fn main() {
    //init underlying network
    let mut network = network::new();
    let incoming_connections_channel = network.setup();

    //init blockchain
    let mut ledger = Ledger { accounts: HashMap::new() };
    let (mut public_key, mut private_key) = gen();
    println!("Gen called");
    println!("Public Key: {}", key_to_string(&public_key));

    //init async input handling
    let input_channel = frontend::init();

    loop {
        //input
        frontend::pull_input(&mut network, &input_channel, &mut public_key, &mut private_key);

        //network
        network.listen_connection(&incoming_connections_channel);
        network.listen_data(blockchain::handle, &mut ledger);
    }
}


