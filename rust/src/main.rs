use bip32::Language;
use clap::Parser;

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

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// number of threads to use
    /// default is 16
    #[arg(long)]
    threads: Option<usize>,

    /// number of jobs to dispatch
    /// default is 10,000
    #[arg(long)]
    jobs: Option<usize>,

    /// number of attempts per job
    /// default is 100
    #[arg(long)]
    each: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let num_threads: usize = args.threads.unwrap_or(16);
    let num_jobs: usize = args.jobs.unwrap_or(1_000);
    let attempts_per_job: usize = args.each.unwrap_or(1_000);

    setup_logger().expect("Failed to set up logger");
    info!(
        "Using {} threads, {} jobs, {} attempts per job",
        num_threads, num_jobs, attempts_per_job
    );

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

    info!("Best: {}", best_address);
}
