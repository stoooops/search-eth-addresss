use bip32::Language;

use crate::{
    criteria::{CriteriaPredicate, LessThanCriteria},
    crypto::{AddressGenerator, MnemonicAddressGenerator},
    logger::setup_logger,
    randnum::{NumberGenerator, RandNumberGenerator},
    search::ThreadPoolSearcher,
};

mod criteria;
mod crypto;
mod logger;
mod randnum;
mod search;
use log::info;

fn main() {
    setup_logger().expect("Failed to set up logger");
    info!("Starting vanitygen...");
    let num_threads = 20;
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

    info!("Best address found: {}", best_address);
    info!("Program finished");
}
