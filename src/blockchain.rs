use crate::structures::{SignedTransaction, Transaction, Message, Ledger};
use crate::crypto::gen;
use rsa::{RSAPublicKey, RSAPrivateKey};
use std::collections::HashMap;

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