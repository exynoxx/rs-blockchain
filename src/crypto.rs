use crypto::ed25519::{keypair, signature, verify as crypto_verify};
use crypto::sha3::Sha3;
use rand::Rng;

pub fn gen() -> ([u8; 32], [u8; 64]) {
    println!("a");
    let random_bytes = [1u8, 2u8, 3u8]; //rand::thread_rng().gen::<[u8; 32]>();
    let (sk, pk) = keypair(&random_bytes);
    return (pk, sk);
}

pub fn sign(msg: &[u8], sk: &[u8]) -> [u8; 64] {
    println!("b");
    return signature(msg, sk);
}

pub fn verify(msg: &[u8], pk: &[u8], signature: &[u8]) -> bool {
    println!("c");
    return crypto_verify(msg, pk, signature);
}

pub fn sha3(msg: &[u8]) -> [u8; 64] {
    return sha3(msg);
}
