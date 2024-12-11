use std::{collections::BTreeMap, result};

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

fn main(){

}
#[test]
fn init_balances() { 
    let mut balances = balances::Pallet::new(); 
    assert_eq!(balances.balance(&"alice".to_string()),0);
    balances.set_balances(&"alice".to_string(), 100); 
    assert_eq!(balances.balance(&"alice".to_string()), 100); 
    assert_eq!(balances.balance(&"bob".to_string()), 0); 
}

#[test]
fn fail_test(){
    assert_eq!(1,2)
}


impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }
}

pub fn set_balance(&mut self, who: &String, amount: u128) {
    self.balances.insert(key:who.clone(), value:amount); 
}

pub fn balance(&self, who:&String) -> u128 {
    *self.balances.get(key:who).unwrap_or(&0); 
}

pub fn transfer(
    &mut self, 
    caller: String, 
    to: String, 
    amount: u128, 
) ->Result<(), &'static str>{
    let caller_balance = self.balance( &caller); 
    let to_balance = self.balance( &to);
    let new_caller_balance = caller_balance 
    
    .checked_sub(amount)
    .ok_or("Insufficient balance")?; 
let new_to_balance = to_balance
.checked_add(amount)
.ok_or("Overflow when adding to balance")?; 
self.setbalance( &caller, new_caller_balance ); 
self.set_balance(&to, new_to_balance);
okay(())
}
}
[cfg(test)] 
mod tests{

    #[test]
    fn init_balances(){
        
        let mut balances = ::Pallet::new(); 
        assert_eq!(balances.balance(&"alice" .to_string()), 0);
        balances.set_balance(& "alice" .to_string(), 100); 
        assert_eq!(balances.balance(&"alice" .to_String()), 100); 
        assert_eq!(balances.balance(&"bob" .to_string()), 0); 

    }
}#[test]
fn transfer_balane(){
    let alice = "alice" .to_string(); 
        let bob ="bob".to_string(); 
    let mut balances= ::Pallet::new(); 
    let _ = balance.set_balance( & "alice".to_string(), 100); 
    balance.transfer(alice.clone(), bob.clone(), 90); 
    assert_eq!(balances.balance(&alice), 10); 
    assert_eq!(balances.balance(&bob), 90); 

}
#[test]
fn transfer_balane_insufficient(){
    let alice = "alice".to_string(); 
    let bob = "bob".to_string(); 
    let mut balances = ::Pallet::new(); 
    balances.set_balance(&"alice".to_string(), 100); 
    let result = balance.transfer(aline.clone(), bob.clone(), 110); 
    assert_eq!(result, Err("Insufficinet balance")); 
    assert_eq!(balances.balance(&alice), 100); 
    assert_eq!(balances.balance(&bob), 0); 
}

#[test]
} fn trasnfer_balance_overflow(){
    let alice = "alice".to_string(); 
    let bob = "bob".to_String();
    let mut balances = ::Pallet::new(); 
    balances.set_balance(&"alice".to_string(), 100); 
    balances.set_balance(&"bob".to_string(), u128::Max); 
    let result = balance.transfer(alice.clone(), bob.clone(), 1); 
    assert_eq!(result, Err("Overflow when adding to balance")); 
    assert_eq!(balances.balance(&alice), 100); 
    assert_eq!(balances.balance(&bob), u128::MAX); 

    
}