#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn pause_toggle() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[]);
    let admin = Address::generate(&env);
    env.mock_all_auths();
    let (p1, p2, p3) = env.as_contract(&contract_id, || {
        let p1 = is_paused(&env);
        pause(&env, &admin);
        let p2 = is_paused(&env);
        unpause(&env, &admin);
        let p3 = is_paused(&env);
        (p1, p2, p3)
    });
    assert!(!p1);
    assert!(p2);
    assert!(!p3);
}
