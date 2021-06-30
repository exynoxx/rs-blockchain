use std::net::{TcpStream, Ipv4Addr};
use std::sync::Mutex;
use std::sync::Arc;
use bincode::{deserialize, serialize};
use crate::crypto::{gen, sign, verify, sha3};

mod network;
mod crypto;


struct Ledger {
    accounts: HashMap<[u8; 64], i64> //public key -> account balance
}

impl Ledger {
    pub fn update_account(&mut self, pk: &[u8], amount: i64) {
        *self.accounts.entry(pk).or_insert(0) += amount;
    }
}

struct Transaction {
    transactionID: u64,
    from: [u8; 64],
    to: [u8; 64],
    amount: i64,
    signature: [u8; 64],
}

struct Block {
    transactions: Vec<Transaction>,
    previous_hash: [u8; 64],
    signature: [u8; 64],
}




fn main() {
    /*let mut network = network::Network{connections: Vec::new(), data_handler: handle };
    let rx = network.setup();
    network.event_loop(&rx);*/

    let (pk, sk) = gen();
    let msg = "yo".as_bytes();
    let sign = sign(&msg, &sk);
    println!("{}", verify(&msg, &pk, &sign));
}


fn handle(data: &[u8]) {
    let d: String = deserialize(data).unwrap();
    println!("handling data {:?}", d);
}
