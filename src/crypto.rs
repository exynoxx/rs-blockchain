use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};
use rsa::hash::Hash::SHA2_256;

pub fn test() {
    let mut rng = OsRng;
    let bits = 1024;
    let (public_key,private_key) = gen();
    let msg = "message".as_bytes();
    println!("{}", verify(&msg, &sign(&msg, &private_key), &public_key));
}

pub fn gen() -> (RSAPublicKey, RSAPrivateKey) {
    let mut rng = OsRng;
    let bits = 1024;
    let private_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RSAPublicKey::from(&private_key);
    return (public_key, private_key);
}

pub fn sign(data: &[u8], sk: &RSAPrivateKey) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hashed = hasher.finalize();
    let padding = PaddingScheme::new_pkcs1v15_sign(Some(SHA2_256));
    let signature = sk.sign(padding, &hashed).expect("failed to sign");
    return signature;
}

pub fn verify(data: &[u8], signature: &Vec<u8>, pk: &RSAPublicKey) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hashed = hasher.finalize();
    let padding = PaddingScheme::new_pkcs1v15_sign(Some(SHA2_256));
    return match pk.verify(padding, &hashed, &signature) {
        Ok(_) => true,
        _ => false
    };
}

pub fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    return hasher.finalize().to_vec();
}