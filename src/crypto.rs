use rand_core::OsRng;
use elgamal_ristretto::public::PublicKey;
use elgamal_ristretto::private::SecretKey;

extern crate rand;
extern crate elgamal_ristretto;
use curve25519_dalek_fiat::ristretto::RistrettoPoint;
use curve25519_dalek_fiat::scalar::Scalar;


pub fn gen() -> (PublicKey, SecretKey) {
    let mut csprng = OsRng;
    // Generate key pair
    let sk = SecretKey::new(&mut csprng);
    let pk = PublicKey::from(&sk);
    return (pk, sk);
}

pub fn encode(msg: &[u8]) -> RistrettoPoint{
    return RistrettoPoint::hash_from_bytes::<sha3>(msg);
}

pub fn sign(msg: &[u8], sk: &SecretKey) -> (Scalar, RistrettoPoint) {
    println!("b");
    let m = encode(msg);
    return sk.sign(&m);
}

pub fn verify(msg: &[u8], pk: &PublicKey, signature: &(Scalar, RistrettoPoint)) -> bool {
    println!("c");
    let m = encode(msg);
    return pk.verify_signature(&m, signature);
}

pub fn sha3(msg: &[u8]) -> [u8; 64] {
    return sha3(msg);
}
