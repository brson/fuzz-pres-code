#![no_std]

use soroban_sdk::{
    contract, contractimpl,
    Env, Address,
};

use subcontract::SubContractClient;

#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn defer_add(
        env: Env,
        subcontract_address: Address,
        amount: i128
    ) -> i128 {
        let client = SubContractClient::new(&env, &subcontract_address);
        client.add(&amount)
    }
}
