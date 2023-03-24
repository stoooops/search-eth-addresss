use bip32::Language;

use crate::criteria::{CriteriaPredicate, LessThanCriteria};
use crate::crypto::{AddressGenerator, MnemonicAddressGenerator};
use crate::randnum::{NumberGenerator, RandNumberGenerator};
use crate::search::Searcher;

mod criteria;
mod crypto;
mod randnum;
mod search;

fn main() {
    let rng: Box<dyn NumberGenerator> = Box::new(RandNumberGenerator {});
    let address_generator: Box<dyn AddressGenerator> = Box::new(MnemonicAddressGenerator {
        language: Language::English,
    });
    let criteria: Box<dyn CriteriaPredicate> = Box::new(LessThanCriteria {});

    let mut searcher = Searcher::new(rng, address_generator, criteria, 1000);
    let best = searcher.run();
    println!("best:         {}", best);
}
