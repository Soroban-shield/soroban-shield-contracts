#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn pause_toggle() {
    let env = Env::default();
    let admin = Address::generate(&env);
    env.mock_all_auths();
    assert!(!is_paused(&env));
    pause(&env, &admin);
    assert!(is_paused(&env));
    unpause(&env, &admin);
    assert!(!is_paused(&env));
}
