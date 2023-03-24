use rand_core::RngCore;

/// Source entropy for a BIP39 mnemonic phrase
pub type Entropy = [u8; 32];

pub trait NumberGenerator {
    fn generate(&mut self) -> Entropy;

    fn clone_box(&self) -> Box<dyn NumberGenerator>;
}

#[derive(Copy, Clone)]
pub struct RandNumberGenerator {}

impl NumberGenerator for RandNumberGenerator {
    fn generate(&mut self) -> Entropy {
        let mut entropy = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut entropy);
        entropy
    }

    fn clone_box(&self) -> Box<dyn NumberGenerator> {
        Box::new(*self)
    }
}
