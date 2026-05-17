#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn two_step_transfer() {
    let env = Env::default();
    let initial_owner = Address::generate(&env);
    let new_owner = Address::generate(&env);
    env.mock_all_auths();
    initialize(&env, &initial_owner);
    transfer_ownership(&env, &new_owner);
    accept_ownership(&env);
    assert_eq!(owner(&env), new_owner);
}
