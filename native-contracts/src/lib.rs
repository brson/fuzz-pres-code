#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn install_subcontract(env: Env) {
    }

    pub fn say_hello(env: Env, amount: i128) {
    }
}
