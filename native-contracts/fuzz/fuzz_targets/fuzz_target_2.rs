#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::{Env, Address};
use soroban_sdk::testutils::Address as _;

mod mycontract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/mycontract.wasm",
    );
}

mod subcontract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/subcontract.wasm",
    );
}

fuzz_target!(|amount: i128| {
    let env = Env::default();
    env.budget().reset_unlimited();

    let mycontract_address = Address::random(&env);
    let subcontract_address = Address::random(&env);

    env.register_contract_wasm(&mycontract_address, mycontract::WASM);
    env.register_contract_wasm(&subcontract_address, subcontract::WASM);

    let client = mycontract::Client::new(&env, &mycontract_address);

    let new_amount = client.defer_add(&subcontract_address, &amount);
    assert_eq!(new_amount, amount.saturating_add(1));
});
