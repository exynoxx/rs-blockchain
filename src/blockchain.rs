use bincode::{deserialize, serialize};
use rsa::{RSAPrivateKey, RSAPublicKey};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::sync;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::crypto;
use crate::crypto::{gen, key_to_string};
use crate::network::{Message, Network};

pub struct Ledger {
    pub accounts: HashMap<Vec<u8>, u64> //public key -> account balance
}

impl Ledger {
    pub fn update_account(&mut self, pk: &Vec<u8>, amount: i64) {
        match self.accounts.get_mut(pk) {
            Some(v) => {
                let mut signed_value = *v as i64;
                signed_value += amount;
                *v = signed_value as u64;
            }
            None => { self.accounts.insert(pk.clone(), 100); }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub transactions: Vec<Transaction>,
    pub previous_signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedBlock {
    pub block: Block,
    pub signature: Vec<u8>,
}

fn signature_valid(t: &SignedTransaction) -> bool {
    return true;
}

fn transaction_valid(t: &Transaction) -> bool {
    return true;
}

pub fn new() -> BlockChain {
    let (pk, sk) = gen();

    println!("Gen called");
    println!("Public Key: {}", key_to_string(&pk));

    let (tx, _) = mpsc::channel();
    let (_, rx) = mpsc::channel();

    return BlockChain {
        ledger: Ledger { accounts: HashMap::new() },
        public_key: pk,
        private_key: sk,
        transaction_channel: tx,
        block_channel: rx,
    };
}

pub struct BlockChain {
    pub ledger: Ledger,
    pub public_key: RSAPublicKey,
    pub private_key: RSAPrivateKey,
    pub transaction_channel: Sender<Transaction>,
    pub block_channel: Receiver<SignedBlock>,
}

fn block_worker(transaction_rx: &Receiver<Transaction>, block_tx: &Sender<SignedBlock>, sk:&RSAPrivateKey) {
    println!("block worker started");

    let mut incoming_transactions: Vec<Transaction> = vec![];

    let block_interval = Duration::from_millis(2000);
    let thread_sleep_interval = Duration::from_millis(500);
    let mut last = Instant::now();

    let mut previous_block_signature: Vec<u8> = vec![];

    loop {
        // receive transaction
        while let Ok(t) = transaction_rx.try_recv() {
            incoming_transactions.push(t);
        }

        // construct block after block_interval time
        if last.elapsed() >= block_interval {
            last = Instant::now();
            if incoming_transactions.len() > 0 {
                println!("constructing block");

                //construct block
                let block = Block {
                    transactions: incoming_transactions.clone(),
                    previous_signature: previous_block_signature.clone(),
                };


                let signature = crypto::sign(&serialize(&block).expect("could not serialize"), sk);

                let signed_block = SignedBlock {
                    block:block,
                    signature:signature.clone(),
                };

                block_tx.send(signed_block);

                incoming_transactions.clear();
                previous_block_signature = signature;
            }
        }

        thread::sleep(thread_sleep_interval);
    }
}

impl BlockChain {
    pub fn init(&mut self) {
        let (transaction_tx, transaction_rx): (Sender<Transaction>, Receiver<Transaction>) = mpsc::channel();
        let (block_tx, block_rx): (Sender<SignedBlock>, Receiver<SignedBlock>) = mpsc::channel();

        self.transaction_channel = transaction_tx;
        self.block_channel = block_rx;

        let sk_copy = self.private_key.clone();

        thread::spawn(move || block_worker(&transaction_rx,&block_tx,&sk_copy));
    }

    pub fn register_transaction(&mut self, t: &Transaction) {
        self.transaction_channel.send(t.clone());
        //self.incoming_transactions.push(t.clone());
    }

    pub fn listen_block(&self, network: &mut Network) {
        if let Ok(signed_block) = self.block_channel.try_recv() {
            network.flood_block(&signed_block);
        }
    }
}

pub fn handle(blockchain: &mut BlockChain, msg: &Message) {
    match msg.typ {
        0 => println!("### greetings!!!! empty init transaction --------------"),
        1 => {
            println!("### incoming transaction");
            //Is signature valid? return if not

            let transaction = match &msg.transaction {
                Some(t) => {
                    if !signature_valid(&t) {
                        println!("Invalid signature");
                        return;
                    }
                    &t.transaction
                }
                None => {
                    println!("No signature");
                    return;
                }
            };

            if !transaction_valid(&transaction) {
                println!("Illegal transaction");
                return;
            }

            blockchain.register_transaction(&transaction);
            blockchain.ledger.update_account(&transaction.from, -(transaction.amount as i64));
            blockchain.ledger.update_account(&transaction.to, transaction.amount as i64);
        }
        2 => {
            println!("### incoming block");

        }
        _ => println!("handling data {:?}", msg)
    }

    println!("hash map:");
    for (key, val) in blockchain.ledger.accounts.iter() {
        println!("account X: {}", val);
    }
}