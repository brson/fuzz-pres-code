#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct SubContract;

#[contractimpl]
impl SubContract {
    pub fn add(_env: Env, amount: i128) -> i128 {
        amount.saturating_add(1)
    }
}
