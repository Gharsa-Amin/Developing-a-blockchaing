use num::{One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + AddAssign + Copy; // Ensure AddAssign is required
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    // Constructor to initialize the pallet
    pub fn new() -> Self {
        Pallet {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    // Increment block_number by 1
    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    // Increment the nonce for a specific user
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let current_nonce = self.nonce.entry(who.clone()).or_insert(T::Nonce::zero());
        *current_nonce += T::Nonce::one(); // Now safe to use += because Nonce implements AddAssign
    }

    // Get the nonce for a specific user
    pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }

    // Getter for block_number
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let system: Pallet<TestConfig> = Pallet::new();
        assert_eq!(system.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut system: Pallet<TestConfig> = Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number(), 1);
    }

    #[test]
    fn inc_nonce() {
        let alice = String::from("alice");
        let mut system: Pallet<TestConfig> = Pallet::new();
        system.inc_nonce(&alice);
        system.inc_nonce(&alice);
        assert_eq!(system.get_nonce(&alice), 2);
    }
}
