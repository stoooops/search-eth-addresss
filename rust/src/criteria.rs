pub trait CriteriaPredicate {
    /// Test whether the given address is better than the current best address.
    fn better(&self, address: &str, other: &str) -> bool;

    fn clone_box(&self) -> Box<dyn CriteriaPredicate>;
}

#[derive(Copy, Clone)]
pub struct LessThanCriteria;

impl CriteriaPredicate for LessThanCriteria {
    fn better(&self, address: &str, other: &str) -> bool {
        address < other
    }

    fn clone_box(&self) -> Box<dyn CriteriaPredicate> {
        Box::new(*self)
    }
}
