use crate::pool::SearcherPool;

mod criteria;
mod crypto;
mod pool;
mod randnum;
mod search;

fn main() {
    let num_workers = 40;
    let max_attempts = 100;

    let searcher_pool = SearcherPool::new(num_workers, max_attempts);
    let best_address = searcher_pool.run();

    println!("Best address found: {}", best_address);
}
