use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
    // type AccountId: Debug; // Ensure that AccountId implements Debug
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }

    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(_) => Err("Claim Already Exists"),
            None => {
                self.claims.insert(claim, caller);
                Ok(())
            }
        }
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let claim_owner = self.get_claim(&claim).ok_or("Claim doesn't exist")?;
        if claim_owner != &caller {
            return Err("Caller is not the owner of the claim");
        }
        self.claims.remove(&claim);
        Ok(())
    }
}
pub enum Call<T: Config> {
    CreateClaim { claim: T::Content },
    RevokeClaim { claim: T::Content },
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
            Call::CreateClaim { claim } => self.create_claim(caller, claim),
            Call::RevokeClaim { claim } => self.revoke_claim(caller, claim),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestConfig;

    impl crate::system::Config for TestConfig {
        type AccountId = &'static str; // This implements Debug
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Content = &'static str;
        // type AccountId = &'static str; // `AccountId` implements Debug
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut poe = Pallet::<TestConfig>::new();

        // Create a claim
        let res = poe.create_claim("alice", "my_document");
        assert_eq!(poe.get_claim(&"my_document"), Some(&"alice"));

        // Try to revoke as a non-owner
        let res = poe.revoke_claim("bob", "my_document");
        assert_eq!(res, Err("Caller is not the owner of the claim"));

        // Try to create an existing claim
        let res = poe.create_claim("bob", "my_document");
        assert_eq!(res, Err("Claim Already Exists"));

        // Try to revoke a non-existent claim
        let res = poe.revoke_claim("alice", "non_existent");
        assert_eq!(res, Err("Claim doesn't exist"));

        // Successfully revoke a claim
        let res = poe.revoke_claim("alice", "my_document");
        assert_eq!(res, Ok(()));
        assert_eq!(poe.get_claim(&"my_document"), None);
    }
}
