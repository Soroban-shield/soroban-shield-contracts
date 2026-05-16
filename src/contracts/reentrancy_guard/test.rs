#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn guard_allows_single_entry() {
    let env = Env::default();
    enter(&env);
    exit(&env);
}
