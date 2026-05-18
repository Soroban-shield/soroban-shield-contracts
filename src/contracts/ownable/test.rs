#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn two_step_transfer() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[]);
    let initial_owner = Address::generate(&env);
    let new_owner = Address::generate(&env);
    env.mock_all_auths();
    let result = env.as_contract(&contract_id, || {
        initialize(&env, &initial_owner);
        transfer_ownership(&env, &new_owner);
        accept_ownership(&env);
        owner(&env)
    });
    assert_eq!(result, new_owner);
}
