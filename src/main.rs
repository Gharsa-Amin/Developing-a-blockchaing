mod balances;
use std::collections::BTreeMap;

fn main() {
    println!("Hello, world!");
}

let mut map: BTreeMap<&str, i32> = BTreeMap::new(); 
map.insert(key: "alice", value: 100); 
assert_eq!(map.get(&"alice"), unwrap_or(&0), &100); 
assert_eq!(map.get(&"bob"), None); 

