#![cfg(test)]

use super::*;
use soroban_sdk::{BytesN, Env};

#[test]
fn stores_implementation_hash() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[]);
    let hash = BytesN::from_array(&env, &[1u8; 32]);
    let result = env.as_contract(&contract_id, || {
        set_implementation(&env, &hash);
        current_implementation(&env)
    });
    assert_eq!(result, hash);
}
