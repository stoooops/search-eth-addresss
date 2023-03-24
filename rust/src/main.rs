use bip32::Language;

use crate::{
    criteria::{CriteriaPredicate, LessThanCriteria},
    crypto::{AddressGenerator, MnemonicAddressGenerator},
    randnum::{NumberGenerator, RandNumberGenerator},
    search::ThreadPoolSearcher,
};

mod criteria;
mod crypto;
mod randnum;
mod search;

fn main() {
    let num_threads = 16;
    let num_jobs = 10_000;
    let attempts_per_job = 100;

    let rng: Box<dyn NumberGenerator + Send + Sync> = Box::new(RandNumberGenerator {});
    let address_generator: Box<dyn AddressGenerator + Send + Sync> =
        Box::new(MnemonicAddressGenerator {
            language: Language::English,
        });
    let criteria: Box<dyn CriteriaPredicate + Send + Sync> = Box::new(LessThanCriteria {});
    let searcher_pool = ThreadPoolSearcher::new(
        num_threads,
        num_jobs,
        attempts_per_job,
        rng,
        address_generator,
        criteria,
    );
    let best_address = searcher_pool.run();

    println!("Best address found: {}", best_address);
}
