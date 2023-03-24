use std::sync::{Arc, Mutex, MutexGuard};

use crate::criteria::{CriteriaPredicate, LessThanCriteria};
use crate::crypto::{AddressGenerator, MnemonicAddressGenerator};
use crate::randnum::{NumberGenerator, RandNumberGenerator};
use crate::search::Searcher;
use bip32::Language;
use rayon::prelude::*;

pub struct SearcherPool {
    num_workers: usize,
    max_attempts: usize,
}

impl SearcherPool {
    pub fn new(num_workers: usize, max_attempts: usize) -> Self {
        Self {
            num_workers,
            max_attempts,
        }
    }

    pub fn run(&self) -> String {
        let best_address = Arc::new(Mutex::new(String::from(
            "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
        )));
        (0..self.num_workers).into_par_iter().for_each_with(
            best_address.clone(),
            |best_addr: &mut Arc<Mutex<String>>, _| {
                let rng: Box<dyn NumberGenerator> = Box::new(RandNumberGenerator {});
                let address_generator: Box<dyn AddressGenerator> =
                    Box::new(MnemonicAddressGenerator {
                        language: Language::English,
                    });
                let criteria: Box<dyn CriteriaPredicate> = Box::new(LessThanCriteria {});

                let mut searcher: Searcher =
                    Searcher::new(rng, address_generator, criteria, self.max_attempts);
                let found_address: String = searcher.run();

                let mut best_address_guard: MutexGuard<String> = best_addr.lock().unwrap();
                if found_address < *best_address_guard {
                    *best_address_guard = found_address;
                    println!("New best address found: {}", *best_address_guard);
                }
            },
        );
        let best_address_guard: MutexGuard<String> = best_address.lock().unwrap();
        best_address_guard.clone()
    }
}
