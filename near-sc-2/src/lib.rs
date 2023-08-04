use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::{near_bindgen, PanicOnDefault, BorshStorageKey};
// use near_sdk::env;
use near_sdk::store::*;

#[derive(Debug, BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
struct Contract {
    number: u8,
    list: Vector<u8>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Vector,    
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            number: 0,
            list: Vector::new(StorageKey::Vector),
        }
    }

    pub fn get_number(&self) -> u8 {
        self.number
    }

   // #[private] //this entry can be called from action (contract owner)
    pub fn increment(&mut self) {
        self.number += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
