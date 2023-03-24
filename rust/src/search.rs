use crate::criteria::CriteriaPredicate;
use crate::crypto::AddressGenerator;
use crate::randnum::NumberGenerator;

pub struct Searcher<'a> {
    number_generator: Box<dyn NumberGenerator + 'a>,
    address_generator: Box<dyn AddressGenerator + 'a>,
    criteria_predicate: Box<dyn CriteriaPredicate + 'a>,
    max_attempts: usize,
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

    pub fn run(&mut self) -> String {
        let input_num = self.number_generator.generate();
        let address = self.address_generator.generate(input_num).unwrap();
        let mut best_address = address;
        for _ in 0..self.max_attempts {
            let entropy = self.number_generator.generate();
            let address = self.address_generator.generate(entropy).unwrap();
            if self.criteria_predicate.better(&address, &best_address) {
                best_address = address;
            }
        }
        best_address
    }
}
