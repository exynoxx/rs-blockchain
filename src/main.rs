use std::net::{TcpStream, Ipv4Addr, SocketAddr, IpAddr};
use std::sync::Mutex;
use std::sync::Arc;
use bincode::{deserialize, serialize};
use std::collections::HashMap;

mod network;
mod frontend;

struct Ledger {
    accounts: HashMap<[u8; 64], i64> //public key -> account balance
}

impl Ledger {
    pub fn update_account(&mut self, pk: &[u8], amount: i64) {
        //*self.accounts.entry(pk).or_insert(0) += amount;
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


fn handle(network: &network::Network, msg: &String) {
    println!("handling data {:?}", msg);
}


fn main() {
    let mut network = network::new(handle);
    let rx_incoming_connections = network.setup();

    //async input handling
    let rx_input = frontend::init();

    loop {

        //input
        match rx_input.try_recv() {
            Ok(line) => match line[0].as_str() {
                "flood" => {
                    //let transaction = 0; //Transaction {};
                    network.flood(&line[1]);
                }
                _=> ()
            }
            Err(t) => ()
        }

        //network
        network.event_loop(&rx_incoming_connections);
    }


    /*let (pk,sk) = gen();

    let msg = "yo".as_bytes();
    let s = sign(&msg, &sk);
    println!("{}", verify(&msg,&pk,&s));*/
}


