#![no_std]
#![allow(non_snake_case)]
#![feature(alloc)]
#![feature(proc_macro)]

extern crate parity_hash;
extern crate pwasm_std;
extern crate pwasm_ethereum;
extern crate alloc;
extern crate pwasm_abi;
extern crate pwasm_abi_derive;
/// Bigint used for 256-bit arithmetic
extern crate bigint;

pub mod token {
    use parity_hash::{H256, Address};
    use pwasm_ethereum;
    use bigint::U256;

    // eth_abi is a procedural macros https://doc.rust-lang.org/book/first-edition/procedural-macros.html
    use pwasm_abi_derive::eth_abi;
    use alloc::Vec;

    static TOTAL_SUPPLY_KEY: H256 = H256([2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);

    #[eth_abi(TokenEndpoint, TokenClient)]
    pub trait TokenInterface {
        /// The constructor
        fn constructor(&mut self, _total_supply: U256);
        /// Total amount of tokens
        #[constant]
        fn totalSupply(&mut self) -> U256;
        /// What is the balance of a particular account?
        #[constant]
        fn balanceOf(&mut self, _owner: Address) -> U256;
        /// Transfer the balance from owner's account to another account
        fn transfer(&mut self, _to: Address, _amount: U256) -> bool;
        /// Event declaration
        #[event]
        fn Transfer(&mut self, indexed_from: Address, indexed_to: Address, _value: U256);
    }

    pub struct TokenContract;

    impl TokenInterface for TokenContract {
        fn constructor(&mut self, total_supply: U256) {
            // Set up the total supply for the token
            pwasm_ethereum::write(&TOTAL_SUPPLY_KEY, &total_supply.into());
        }

        fn totalSupply(&mut self) -> U256 {
            pwasm_ethereum::read(&TOTAL_SUPPLY_KEY).into()
        }

        fn balanceOf(&mut self, owner: Address) -> U256 {
            read_balance_of(&owner)
        }

        fn transfer(&mut self, to: Address, amount: U256) -> bool {
            let sender = pwasm_ethereum::sender();
            let senderBalance = read_balance_of(&sender);
            let recipientBalance = read_balance_of(&to);
            if amount == 0.into() || senderBalance < amount {
                false
            } else {
                let new_sender_balance = senderBalance - amount;
                let new_recipient_balance = recipientBalance + amount;
                pwasm_ethereum::write(&balance_key(&sender), &new_sender_balance.into());
                pwasm_ethereum::write(&balance_key(&to), &new_recipient_balance.into());
                self.Transfer(sender, to, amount);
                true
            }
        }
    }

    // Reads balance by address
    fn read_balance_of(owner: &Address) -> U256 {
        pwasm_ethereum::read(&balance_key(owner)).into()
    }

    // Generates a balance key for some address.
    // Used to map balances with their owners.
    fn balance_key(address: &Address) -> H256 {
        let mut key = H256::from(address);
        key[0] = 1; // just a naiive "namespace";
        key
    }
}
// Declares the dispatch and dispatch_ctor methods
use pwasm_abi::eth::EndpointInterface;

#[no_mangle]
pub fn call() {
    let mut endpoint = token::TokenEndpoint::new(token::TokenContract{});
    // Read http://solidity.readthedocs.io/en/develop/abi-spec.html#formal-specification-of-the-encoding for details
    pwasm_ethereum::ret(&endpoint.dispatch(&pwasm_ethereum::input()));
}

#[no_mangle]
pub fn deploy() {
    let mut endpoint = token::TokenEndpoint::new(token::TokenContract{});
    endpoint.dispatch_ctor(&pwasm_ethereum::input());
}
