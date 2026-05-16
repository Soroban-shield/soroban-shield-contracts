#![cfg(test)]

use super::*;
use soroban_sdk::{BytesN, Env};

#[test]
fn stores_implementation_hash() {
    let env = Env::default();
    let hash = BytesN::from_array(&env, &[1u8; 32]);
    set_implementation(&env, &hash);
    assert_eq!(current_implementation(&env), hash);
}
