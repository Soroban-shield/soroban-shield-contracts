#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn allows_calls_under_limit() {
    let env = Env::default();
    let caller = Address::generate(&env);
    configure(&env, 3, 60);
    assert!(check_and_record(&env, &caller, 1).is_ok());
}
