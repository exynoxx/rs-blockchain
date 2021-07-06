use std::net::{TcpStream, Ipv4Addr, SocketAddr, IpAddr};
use std::sync::Mutex;
use std::sync::Arc;
use bincode::{deserialize, serialize};
use std::collections::HashMap;
use crate::crypto::{test, gen, sign, verify, deserialize_key, key_to_string, string_to_key};
use crate::structures::{Transaction, Message, SignedTransaction, Ledger};
use rsa::{PublicKeyEncoding, PrivateKeyEncoding, PublicKeyParts, BigUint, RSAPrivateKey, RSAPublicKey};
use serde::{Serialize, Deserialize};
use std::borrow::Borrow;
use std::thread;

mod network;
mod frontend;
mod crypto;
mod structures;

fn signature_valid(t: &SignedTransaction) -> bool {
    return true;
}

fn handle(blockchain: &mut Ledger, msg: &Message) {
    match msg.typ {
        0 => println!("greetings"),
        1 => {
            println!("incoming transaction");
            //Is signature valid? return if not

            let transaction = match &msg.transaction {
                Some(t) => {
                    if !signature_valid(&t) {
                        println!("Invalid signature");
                        return;
                    }
                    &t.transaction
                }
                None => {println!("No signature");return}
            };

            blockchain.update_account(&transaction.from,-(transaction.amount as i64));
            blockchain.update_account(&transaction.to,transaction.amount as i64);
        }
        _ => println!("handling data {:?}", msg)
    }

    println!("hash map {:?}", blockchain.accounts);
}

fn main() {


    //init underlying network
    let mut network = network::new(handle);
    let incoming_connections_channel = network.setup();

    //init blockchain
    let mut ledger = Ledger{ accounts: HashMap::new() };
    let (mut public_key, mut private_key) = gen();
    println!("Gen called");
    println!("Public Key: {}", key_to_string(&public_key));

    //init async input handling
    let input_channel = frontend::init();

    loop {
        //input
        frontend::pull_input(&mut network, &input_channel,&mut public_key,&mut private_key);

        //network
        network.listen_connection(&incoming_connections_channel);
        network.listen_data(&mut ledger);
    }
}


