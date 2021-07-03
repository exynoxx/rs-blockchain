use std::net::{TcpStream, Ipv4Addr, SocketAddr, IpAddr};
use std::sync::Mutex;
use std::sync::Arc;
use bincode::{deserialize, serialize};
use std::collections::HashMap;
use crate::crypto::{test, gen,sign,verify};
use crate::structures::{Transaction, Message, SignedTransaction};
use rsa::{PublicKeyEncoding, PrivateKeyEncoding, PublicKeyParts, BigUint, RSAPrivateKey, RSAPublicKey};

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
    let mut private_key = None;
    let mut public_key = None;

    //async input handling
    let rx_input = frontend::init();

    loop {

        //input
        let line = rx_input.try_recv().unwrap_or(vec!["-1".to_string()]);
        match line[0].as_str() {
            "gen" => {
                let (pk, sk) = gen();
                println!("Public Key: {:?} {:?}", pk.e().to_str_radix(16), pk.n().to_str_radix(16));
                //println!("Private Key: {:?}",sk.to_pkcs8().expect("Could not extract string from sk"));
                private_key = Some(sk);
                public_key = Some(pk);
            }

            "transfer" => {
                let amount = line[1].parse::<u64>().unwrap();
                let to_str_e = line[2];
                let to_str_n = line[3];
                let receiver_pk = RSAPublicKey { n: BigUint::from_str_radix(to_str_n,16), e: BigUint::from_str_radix(to_str_e,16) };

                let mut t = Transaction {
                    from: private_key.unwrap().to_pkcs8().expect("error"),
                    to: receiver_pk.to_pkcs8().expect("error"),
                    amount: amount
                };

                let transaction = SignedTransaction {
                    transaction: t,
                    signature: crypto::sign(&serialize(&t), &private_key)
                };

                network.flood_transaction(&transaction);


            }

            "flood" => network.flood_transaction(&Transaction {
                from: vec![],
                to: vec![],
                amount: 0,
            }),
            _ => ()
        }

        //network
        network.listen_connection(&rx_incoming_connections);
        network.listen_data();
    }
}


