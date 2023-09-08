#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::{Env, Address};
use soroban_sdk::testutils::Address as _;

use mycontract::{MyContract, MyContractClient};
use subcontract::SubContract;

fuzz_target!(|amount: i128| {
    let env = Env::default();

    let mycontract_address = Address::random(&env);
    let subcontract_address = Address::random(&env);

    env.register_contract(&mycontract_address, MyContract);
    env.register_contract(&subcontract_address, SubContract);

    let client = MyContractClient::new(&env, &mycontract_address);

    let new_amount = client.defer_add(&subcontract_address, &amount);

    assert_eq!(new_amount, amount.saturating_add(1));
});
