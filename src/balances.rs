use num::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;
pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient Balance")?;

        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow when adding to balance")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);
        Ok(())
    }
}

pub enum Call<T: Config> {
    Transfer {
        to: T::AccountId,
        amount: T::Balance,
    },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        call: Self::Call,
    ) -> crate::support::DispatchResult {
        match call {
            Call::Transfer { to, amount } => self.transfer(caller, to, amount)?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use crate::{balances::Pallet, system, types};

    struct TestConfig;
    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }
    impl super::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();
        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();
        balances.set_balance(&alice, 100);
        let _ = balances.transfer(alice.clone(), bob.clone(), 90);
        assert_eq!(balances.balance(&alice), 10);
        assert_eq!(balances.balance(&bob), 90);
    }

    #[test]
    fn transfer_balance_insufficient() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();
        let result = balances.transfer(alice.clone(), bob.clone(), 100);
        assert_eq!(result, Err("Insufficient Balance"));
        assert_eq!(balances.balance(&alice), 0);
        assert_eq!(balances.balance(&bob), 0);
    }

    #[test]
    fn transfer_balance_overflow() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();
        balances.set_balance(&"alice".to_string(), 100);
        balances.set_balance(&"bob".to_string(), u128::MAX);
        let result = balances.transfer(alice.clone(), bob.clone(), 1);
        assert_eq!(result, Err("Overflow when adding to balance"));
        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), u128::MAX);
    }
}
