use std::io;
use std::sync::mpsc;
use std::thread;
use crate::crypto::{gen, key_to_string, string_to_key};
use crate::crypto;
use bincode::serialize;
use rsa::{RSAPublicKey, RSAPrivateKey};
use crate::network::Network;
use crate::blockchain::{Transaction, SignedTransaction};

//read line from STD-IN
fn readline() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    return buffer.trim().split(' ').map(|s| s.to_string()).collect();
}

//thread read lines from stdin and send them in tunnel, return tunnel to caller
pub(crate) fn init() -> mpsc::Receiver<Vec<String>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            tx.send(readline()); //frontend.rs
        }
    });
    return rx;
}


pub fn pull_input(network: &mut Network, input_channel: &mpsc::Receiver<Vec<String>>, public_key: &mut RSAPublicKey, private_key: &mut RSAPrivateKey) {
    let line = input_channel.try_recv().unwrap_or(vec!["-1".to_string()]);
    match line[0].as_str() {
        "gen" => {
            let (pk, sk) = gen();
            println!("Public Key: {}", key_to_string(&pk));

            //println!("Private Key: {:?}",sk.to_pkcs8().expect("Could not extract string from sk"));
            *private_key = sk.clone();
            *public_key = pk.clone();
        }

        "transfer" => {
            let amount = line[1].parse::<u64>().unwrap();
            let receiver_pk = string_to_key(&line[2]);

            let pk = &public_key;
            let sk = &private_key;

            let t = Transaction {
                from: crypto::serialize_key(&pk),
                to: crypto::serialize_key(&receiver_pk),
                amount,
            };

            let signature = crypto::sign(&serialize(&t).expect("could not serialize"), &sk);

            let transaction = SignedTransaction {
                transaction: t,
                signature,
            };

            network.flood_transaction(&transaction);
        }

        "flood" => network.flood_transaction(&SignedTransaction {
            transaction: Transaction {
                from: vec![],
                to: vec![],
                amount: 0,
            },
            signature: vec![],
        }),
        _ => ()
    }
}