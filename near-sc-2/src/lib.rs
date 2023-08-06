use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, PanicOnDefault, BorshStorageKey, AccountId, Promise, require};
use near_sdk::store::*;

#[derive(Debug, BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
struct Contract {
    balances: LookupMap<AccountId, u128>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Balances,    
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            balances: LookupMap::new(StorageKey::Balances)
        }
    }

    pub fn balance_of(&self, account_id: AccountId) -> U128 {
        U128(*self.balances.get(&account_id).unwrap_or(&0u128))
    }

   // #[private] //this entry can be called from action (contract owner)
   #[payable]
    pub fn mint(&mut self) {
        let account_id = env::predecessor_account_id();
        let amount = env::attached_deposit();
        let current_balance = self.balance_of(account_id.clone()).0;
        self.balances
            .set(account_id, Some(current_balance + amount));
    }

    pub fn burn(&mut self, amount: U128) -> Promise {
        let account_id = env::predecessor_account_id();
        let current_balance = self.balance_of(account_id.clone()).0;
        self.balances
            .set(account_id.clone(), Some(current_balance.checked_sub(amount.0).
            unwrap_or_else(|| {
                env::panic_str("You tried to withdraw more tokens then you own.");
            })));

        Promise::new(account_id).transfer(amount.0)
    }

    pub fn transfer(&mut self, receiver_id: AccountId, amount: U128) {
         let sender_id = env::predecessor_account_id();
         
         require!(
            sender_id != receiver_id,
            "Sender and Receiver can not be the same",
         );

         let sender_current_balance = self.balance_of(sender_id.clone()).0;
         let receiver_current_balance = self.balance_of(receiver_id.clone()).0;
         let sender_new_balance = sender_current_balance.checked_sub(amount.0).
         unwrap_or_else(|| {
            env::panic_str("You tried to send more tokens then you own.");
         });
         let receiver_new_balance = receiver_current_balance.checked_add(amount.0).
         unwrap_or_else(|| {
            env::panic_str("Your blance overflowed!");
         });
         self.balances.set(sender_id, Some(sender_new_balance));
         self.balances.set(receiver_id, Some(receiver_new_balance));
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
