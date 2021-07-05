use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use bincode;
use bincode::Options;


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

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: usize,
    pub typ: usize,
    pub transaction: Option<SignedTransaction>,
    pub block: Option<Block>,
}