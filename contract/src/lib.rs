use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
pub use near_sdk::collections::vector::Vector;
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};
use std::convert::TryInto;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {

    pub authority: Option<AccountId>,
    pub value : u8,
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct ValueUpdateArgs {
    pub new_val: u8,
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(owner: AccountId) -> Self {
        Contract {
            authority: Some(owner),
            value: 0,
        }
    }

    pub fn do_something(&mut self, ctx: ValueUpdateArgs) {
        self.value = ctx.new_val;
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

}
