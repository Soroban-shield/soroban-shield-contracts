#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};

#[test]
fn grant_and_check_role() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[] as &[u8]);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    env.mock_all_auths();
    let role = symbol_short!("minter");
    let result = env.as_contract(&contract_id, || {
        grant_role(&env, &role, &user, &admin);
        has_role(&env, &role, &user)
    });
    assert!(result);
}
