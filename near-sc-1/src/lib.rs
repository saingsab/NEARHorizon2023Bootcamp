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
            list: Vector::new(0),
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

// cfg(test) this will exluded in the wasm file when build to deploy
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let contract = Contract {
            number: 5,
            list: Vector::new(0),
        };

        let serialization = contract.try_to_vec().unwrap();
        println!("{serialization:#?}");
    }

    #[test]
    fn test_get_number() {
        let contract = Contract {
            number: 5,
            list: Vector::new(0),
        };
        assert_eq!(5, contract.get_number());
    }

    #[test]
    fn test_inc_number() {
        let mut contract = Contract {
            number: 1,
            list: Vector::new(0),
        };
        contract.increment();
        assert_eq!(2, contract.get_number());
    }
}
