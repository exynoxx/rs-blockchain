use crate::crypto::gen;
use rsa::{RSAPublicKey, RSAPrivateKey};
use std::collections::HashMap;
use crate::network::Message;
use bincode::{deserialize, serialize};
use serde::{Serialize, Deserialize};

pub struct Ledger {
    pub accounts: HashMap<Vec<u8>, u64> //public key -> account balance
}

impl Ledger {
    pub fn update_account(&mut self, pk: &Vec<u8>, amount: i64) {
        match self.accounts.get_mut(pk){
            Some(v) => {
                let mut signed_value = *v as i64;
                signed_value += amount;
                *v = signed_value as u64;
            },
            None => {self.accounts.insert(pk.clone(),100);},

        }
    }
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Transaction {
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: Vec<u8>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub transactions: Vec<Transaction>,
    pub previous_signature: Vec<u8>,
    pub signature: Vec<u8>,
}






fn signature_valid(t: &SignedTransaction) -> bool {
    return true;
}

fn transaction_valid(t: &Transaction) -> bool {
    return true;
}

pub fn new() -> BlockChain{
    let (pk,sk) = gen();
    return BlockChain {
        incoming_transactions: vec![],
        ledger: Ledger { accounts: HashMap::new() },
        public_key: pk,
        private_key: sk
    }
}

pub struct BlockChain {
    pub incoming_transactions: Vec<Transaction>,
    pub ledger: Ledger,
    pub public_key: RSAPublicKey,
    pub private_key: RSAPrivateKey,
}

impl BlockChain {
    pub fn register_transaction(&mut self,t: &Transaction) {
        self.incoming_transactions.push(t.clone());
    }
}

pub fn handle(blockchain: &mut BlockChain, msg: &Message) {
    match msg.typ {
        0 => println!("greetings!!!! empty init transaction --------------"),
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
                None => {
                    println!("No signature");
                    return;
                }
            };

            if !transaction_valid(&transaction) {
                println!("Illegal transaction");
                return;
            }

            blockchain.ledger.update_account(&transaction.from, -(transaction.amount as i64));
            blockchain.ledger.update_account(&transaction.to, transaction.amount as i64);
        }
        _ => println!("handling data {:?}", msg)
    }

    println!("hash map:");
    for (key, val) in blockchain.ledger.accounts.iter() {
        println!("account X: {}", val);
    }
}