use bip32::Language;

use crate::criteria::{CriteriaPredicate, LessThanCriteria};
use crate::crypto::{AddressGenerator, MnemonicAddressGenerator};
use crate::randnum::{NumberGenerator, RandNumberGenerator};

mod criteria;
mod crypto;
mod randnum;

fn main() {
    let mut rng = RandNumberGenerator {};
    let address_generator: Box<dyn AddressGenerator> = Box::new(MnemonicAddressGenerator {
        language: Language::English,
    });

    let entropy: [u8; 32] = rng.generate();
    let address: String = address_generator.generate(entropy).unwrap();
    let address2: String = address_generator.generate(rng.generate()).unwrap();
    println!("address :     {}", address);
    println!("address2:     {}", address2);

    let criteria: Box<dyn CriteriaPredicate> = Box::new(LessThanCriteria {});
    let better = criteria.better(&address2, &address);
    println!("2 lessthan 1: {}", better);
}
