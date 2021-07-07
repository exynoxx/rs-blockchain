use crate::crypto::{gen, key_to_string, string_to_key};

mod network;
mod frontend;
mod crypto;
mod blockchain;

fn main() {
    //init underlying network
    let mut network = network::new();
    network.setup();

    //init blockchain
    let mut blockchain = blockchain::new();
    blockchain.init();

    //init async input handling
    let input_channel = frontend::init();

    loop {
        //input
        frontend::pull_input(&mut network, &input_channel, &mut blockchain.public_key, &mut blockchain.private_key);

        //network
        network.listen_connection();
        network.listen_data(&mut blockchain,blockchain::handle);

        //blockchain
        blockchain.listen_block(&mut network);
    }
}


