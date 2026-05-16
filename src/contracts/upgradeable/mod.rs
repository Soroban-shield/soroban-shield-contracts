//! WASM proxy upgrade pattern for Soroban.

pub mod contract;
pub mod interface;
pub mod proxy;

#[cfg(test)]
mod test;

use soroban_sdk::{BytesN, Env};

pub use proxy::{current_implementation, is_locked, set_implementation};

pub fn upgrade(env: &Env, new_wasm_hash: &BytesN<32>) {
    if proxy::is_locked(env) {
        panic!("upgrades locked");
    }
    proxy::set_implementation(env, new_wasm_hash);
}

pub fn lock_upgrades(env: &Env) {
    use soroban_sdk::Symbol;
    const LOCKED: Symbol = soroban_sdk::symbol_short!("UPLOCK");
    env.storage().instance().set(&LOCKED, &true);
}
