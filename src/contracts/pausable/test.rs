#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn pause_toggle() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[] as &[u8]);
    let admin = Address::generate(&env);
    env.mock_all_auths();
    let p1 = env.as_contract(&contract_id, || is_paused(&env));
    env.as_contract(&contract_id, || pause(&env, &admin));
    let p2 = env.as_contract(&contract_id, || is_paused(&env));
    env.as_contract(&contract_id, || unpause(&env, &admin));
    let p3 = env.as_contract(&contract_id, || is_paused(&env));
    assert!(!p1);
    assert!(p2);
    assert!(!p3);
}
