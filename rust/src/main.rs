use crate::randnum::{NumberGenerator, RandNumberGenerator};

mod crypto;
mod randnum;

fn main() {
    let mut rng = RandNumberGenerator {};
    let entropy = rng.generate();
    let result = crypto::generate(entropy).unwrap();
    println!("mnemonic: {}", result.mnemonic.phrase());
    println!("address: {}", result.address);
}
