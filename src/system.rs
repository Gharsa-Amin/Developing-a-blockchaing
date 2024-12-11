use std::collections::BTreeMap;

pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    // Constructor to initialize the pallet
    pub fn new() -> Self {
        Pallet {
            block_number: 0, // Initialize block_number to 0
            nonce: BTreeMap::new(),
        }
    }

    // Increment block_number by 1
    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    // Increment the nonce for a specific user
    pub fn inc_nonce(&mut self, who: &String) {
        let nonce = self.nonce.get(who).unwrap_or(&0);
        self.nonce.insert(who.clone(), nonce + 1);
    }

    // Get the nonce for a specific user
    pub fn get_nonce(&self, who: &String) -> u32 {
        *self.nonce.get(who).unwrap_or(&0)
    }

    // Getter for block_number
    pub fn block_number(&self) -> u32 {
        self.block_number
    }
}

#[cfg(test)]
mod test {
    use super::Pallet;

    #[test]
    fn init_system() {
        let system = Pallet::new();
        assert_eq!(system.block_number(), 0); // Check initial block_number is 0
    }

    #[test]
    fn inc_block_number() {
        let mut system = Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number(), 1); // Check block_number increments to 1
    }

    #[test]
    fn inc_nonce() {
        let alice = String::from("alice");
        let mut system = Pallet::new();
        system.inc_nonce(&alice);
        assert_eq!(system.get_nonce(&alice), 1); // Check that the nonce for alice is 1
    }
}
