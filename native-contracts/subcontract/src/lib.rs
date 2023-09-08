#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct SubContract;

#[contractimpl]
impl SubContract {
    pub fn hello(env: Env, amount: i128) {
    }
}
