use crate::search::ThreadPoolSearcher;

mod criteria;
mod crypto;
mod randnum;
mod search;

fn main() {
    let num_threads = 16;
    let num_jobs = 10_000;
    let attempts_per_job = 100;

    let searcher_pool = ThreadPoolSearcher::new(num_threads, num_jobs, attempts_per_job);
    let best_address = searcher_pool.run();

    println!("Best address found: {}", best_address);
}
