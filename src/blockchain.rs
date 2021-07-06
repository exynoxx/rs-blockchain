use crate::structures::{SignedTransaction, Transaction, Message, Ledger};

fn signature_valid(t: &SignedTransaction) -> bool {
    return true;
}

fn transaction_valid(t: &Transaction) -> bool {
    return true;
}

pub(crate) fn handle(blockchain: &mut Ledger, msg: &Message) {
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

            blockchain.update_account(&transaction.from, -(transaction.amount as i64));
            blockchain.update_account(&transaction.to, transaction.amount as i64);
        }
        _ => println!("handling data {:?}", msg)
    }

    println!("hash map:");
    for (key, val) in blockchain.accounts.iter() {
        println!("account X: {}", val);
    }
}