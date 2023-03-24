use bip32::Language;

use crate::crypto::{AddressGenerator, MnemonicAddressGenerator};
use crate::randnum::{NumberGenerator, RandNumberGenerator};

mod crypto;
mod randnum;

fn main() {
    let mut rng = RandNumberGenerator {};
    let address_generator: Box<dyn AddressGenerator> = Box::new(MnemonicAddressGenerator {
        language: Language::English,
    });

    let entropy: [u8; 32] = rng.generate();
    let address: String = address_generator.generate(entropy).unwrap();
    println!("address: {}", address);
}
