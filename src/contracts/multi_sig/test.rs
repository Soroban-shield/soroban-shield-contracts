#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};

#[test]
fn creates_incrementing_ids() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[] as &[u8]);
    let creator = Address::generate(&env);
    env.mock_all_auths();
    let id = env.as_contract(&contract_id, || {
        set_threshold(&env, 2);
        create_proposal(&env, &creator, symbol_short!("upgrade"), 1000)
    });
    assert_eq!(id, 1);
}
