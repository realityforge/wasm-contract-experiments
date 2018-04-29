// Contract doesn't use Rust's standard library
#![no_std]
#![allow(non_snake_case)]
#![feature(proc_macro)]

// `pwasm-ethereum` implements bindings to the runtime
/// Bigint used for 256-bit arithmetic
extern crate bigint;
extern crate parity_hash;
extern crate pwasm_abi;
extern crate pwasm_abi_derive;
extern crate pwasm_ethereum;
extern crate pwasm_std;

// Declares the dispatch and dispatch_ctor methods
use pwasm_abi::eth::EndpointInterface;

pub mod token {
    use bigint::U256;
    use parity_hash::H256;
    // eth_abi is a procedural macros https://doc.rust-lang.org/book/first-edition/procedural-macros.html
    use pwasm_abi_derive::eth_abi;
    use pwasm_ethereum;
    use pwasm_std::Vec;

    static TOTAL_SUPPLY_KEY: H256 = H256([
        2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);

    #[eth_abi(TokenEndpoint, TokenClient)]
    pub trait TokenInterface {
        /// The constructor
        fn constructor(&mut self, _total_supply: U256);
        /// Total amount of tokens
        #[constant]
        fn totalSupply(&mut self) -> U256;
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
    }
}

// The "deploy" will be executed only once on deployment but will not be stored on the blockchain
#[no_mangle]
pub fn deploy() {
    let mut endpoint = token::TokenEndpoint::new(token::TokenContract {});
    endpoint.dispatch_ctor(&pwasm_ethereum::input());
}

// This code will be stored on blockchain
#[no_mangle]
pub fn call() {
    let mut endpoint = token::TokenEndpoint::new(token::TokenContract {});
    // Read http://solidity.readthedocs.io/en/develop/abi-spec.html#formal-specification-of-the-encoding for details
    pwasm_ethereum::ret(&endpoint.dispatch(&pwasm_ethereum::input()));
}
