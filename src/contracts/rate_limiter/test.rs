#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn allows_calls_under_limit() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[] as &[u8]);
    let caller = Address::generate(&env);
    let ok = env.as_contract(&contract_id, || {
        configure(&env, 3, 60);
        check_and_record(&env, &caller, 1).is_ok()
    });
    assert!(ok);
}
