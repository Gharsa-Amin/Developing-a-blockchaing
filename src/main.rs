mod balances;
mod system;

fn main() {
    let mut balances = balance:Pallet::new(); 
    let mut system = system::Pallet::new(); 
    // Create a new BTreeMap with keys of type &str and values of type i32
    // let mut map: BTreeMap<&str, i32> = BTreeMap::new();

    // // Insert a key-value pair into the map
    // map.insert("alice", 100);

    // // Check if "alice" exists in the map and assert the value is 100
    // assert_eq!(map.get(&"alice").unwrap_or(&0), &100);

    // // Check if "bob" does not exist in the map and assert it's None
    // assert_eq!(map.get(&"bob"), None);
}
