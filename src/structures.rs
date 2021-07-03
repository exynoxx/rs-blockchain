use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use bincode;
use bincode::Options;


pub struct Ledger {
    accounts: HashMap<Vec<u8>, i64> //public key -> account balance
}

impl Ledger {
    pub fn update_account(&mut self, pk: &Vec<u8>, amount: i64) {
        //*self.accounts.entry(pk).or_insert(0) += amount;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: usize,
    pub typ: usize,
    pub transaction: Option<Transaction>,
    pub block: Option<Block>,
}