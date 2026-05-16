#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};

#[test]
fn creates_incrementing_ids() {
    let env = Env::default();
    let creator = Address::generate(&env);
    env.mock_all_auths();
    set_threshold(&env, 2);
    let id = create_proposal(&env, &creator, symbol_short!("upgrade"), 1000);
    assert_eq!(id, 1);
}
