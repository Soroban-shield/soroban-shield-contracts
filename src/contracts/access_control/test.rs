#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};

#[test]
fn grant_and_check_role() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    env.mock_all_auths();
    let role = symbol_short!("minter");
    grant_role(&env, &role, &user, &admin);
    assert!(has_role(&env, &role, &user));
}
