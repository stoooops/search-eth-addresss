use crate::criteria::LessThanCriteria;
use crate::crypto::AddressGenerator;
use crate::randnum::{NumberGenerator, RandNumberGenerator};
use crate::{criteria::CriteriaPredicate, crypto::MnemonicAddressGenerator};

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use bip32::{Language, Mnemonic};
use rayon::{current_thread_index, prelude::*, ThreadPool, ThreadPoolBuilder};

pub struct Searcher<'a> {
    number_generator: Box<dyn NumberGenerator + 'a>,
    address_generator: Box<dyn AddressGenerator + 'a>,
    criteria_predicate: Box<dyn CriteriaPredicate + 'a>,
    max_attempts: usize,
}

pub struct SearchResult {
    pub address: String,
    pub seed: [u8; 32],
}

impl<'a> Searcher<'a> {
    pub fn new(
        number_generator: Box<dyn NumberGenerator + 'a>,
        address_generator: Box<dyn AddressGenerator + 'a>,
        criteria_predicate: Box<dyn CriteriaPredicate + 'a>,
        max_attempts: usize,
    ) -> Self {
        Self {
            number_generator,
            address_generator,
            criteria_predicate,
            max_attempts,
        }
    }

    pub fn run(&mut self) -> SearchResult {
        let input_num = self.number_generator.generate();
        let address = self.address_generator.generate(input_num).unwrap();
        let mut best: SearchResult = SearchResult {
            address,
            seed: input_num,
        };
        for _ in 0..self.max_attempts {
            let entropy = self.number_generator.generate();
            let address = self.address_generator.generate(entropy).unwrap();
            if self.criteria_predicate.better(&address, &best.address) {
                best = SearchResult {
                    address,
                    seed: entropy,
                };
            }
        }
        best
    }
}

pub struct ThreadPoolSearcher {
    thread_pool: ThreadPool,
    num_jobs: usize,
    attempts_per_job: usize,
}

impl ThreadPoolSearcher {
    pub fn new(num_threads: usize, num_jobs: usize, attempts_per_job: usize) -> Self {
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .expect("Failed to create thread pool");

        Self {
            thread_pool,
            num_jobs,
            attempts_per_job,
        }
    }

    pub fn run(&self) -> String {
        let best_address = Arc::new(Mutex::new(String::from(
            "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
        )));
        let completed_jobs = Arc::new(AtomicUsize::new(0));
        let num_completed_jobs_log_width = format!("{}", self.num_jobs).len();
        let num_threads_log_width = format!("{}", self.thread_pool.current_num_threads()).len();
        let num_searches_log_width = format!("{}", self.num_jobs * self.attempts_per_job).len();
        let criteria: Box<dyn CriteriaPredicate + Send + Sync> = Box::new(LessThanCriteria {});

        self.thread_pool.install(|| {
            (0..self.num_jobs)
                .into_par_iter()
                .enumerate()
                .for_each_with(
                    best_address.clone(),
                    |best: &mut Arc<Mutex<String>>, (_job_num, _worker_id)| {
                        let rng: Box<dyn NumberGenerator> = Box::new(RandNumberGenerator {});
                        let address_generator: Box<dyn AddressGenerator> =
                            Box::new(MnemonicAddressGenerator {
                                language: Language::English,
                            });

                        // Criteria gets moved here, so we need to clone it
                        // but it's a box so we need to clone the box
                        let mut searcher: Searcher =
                            Searcher::new(rng, address_generator, criteria.clone_box(), self.attempts_per_job);
                        let found: SearchResult = searcher.run();
                        let num_completed_jobs = completed_jobs.fetch_add(1, Ordering::SeqCst) + 1;
                        let num_completed_searches: usize = num_completed_jobs * self.attempts_per_job;

                        let mut best_address_guard: MutexGuard<String> = best.lock().unwrap();
                        if criteria.better(&found.address, &*best_address_guard) {
                            *best_address_guard = found.address;
                            let thread_index = current_thread_index().unwrap_or(0);
                            println!(
                                "Thread #{:twidth$}     Job #{:jwidth$}     Found #{:swidth$}     found     {}",
                                thread_index,
                                num_completed_jobs,
                                num_completed_searches,
                                *best_address_guard,
                                twidth = num_threads_log_width,
                                jwidth = num_completed_jobs_log_width,
                                swidth = num_searches_log_width
                            );
                            let mnemonic: Mnemonic = Mnemonic::from_entropy(found.seed, Language::English);
                            println!("{}", mnemonic.phrase());
                        } else if (num_completed_jobs % 1000) == 0 {
                            let thread_index = current_thread_index().unwrap_or(0);
                            println!(
                                "Thread #{:twidth$}     Job #{:jwidth$}     Found #{:swidth$}     -----     {}",
                                thread_index,
                                num_completed_jobs,
                                num_completed_searches,
                                *best_address_guard,
                                twidth = num_threads_log_width,
                                jwidth = num_completed_jobs_log_width,
                                swidth = num_searches_log_width
                            );
                            let mnemonic: Mnemonic = Mnemonic::from_entropy(found.seed, Language::English);
                            println!("{}", mnemonic.phrase());
                        }
                    },
                );
        });

        let best_address_guard: MutexGuard<String> = best_address.lock().unwrap();
        best_address_guard.clone()
    }

    // fn update_best(
    //     found: SearchResult,
    //     best: &mut Arc<Mutex<String>>,
    //     criteria: &Box<dyn CriteriaPredicate>,
    // ) {
    //     let mut best_address_guard: MutexGuard<String> = best.lock().unwrap();
    //     if criteria.better(&found.address, &*best.lock().unwrap()) {
    //         *best = found;
    //     }
    // }
}
