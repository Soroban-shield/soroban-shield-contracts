#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn guard_allows_single_entry() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[] as &[u8]);
    env.as_contract(&contract_id, || {
        enter(&env);
        exit(&env);
    });
}
