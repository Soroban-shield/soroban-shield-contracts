//! Single-owner access with two-step transfer.

pub mod contract;
pub mod interface;

#[cfg(test)]
mod test;

use soroban_sdk::{Address, Env, Symbol};

pub use contract::{accept_ownership, renounce_ownership, transfer_ownership};

const OWNER: Symbol = soroban_sdk::symbol_short!("OWNER");
pub(crate) const PENDING: Symbol = soroban_sdk::symbol_short!("PEND");

pub fn initialize(env: &Env, owner: &Address) {
    if env.storage().instance().has(&OWNER) {
        panic!("ownable: already initialized");
    }
    env.storage().instance().set(&OWNER, owner);
}

pub fn owner(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&OWNER)
        .expect("ownable: not initialized")
}

pub fn only_owner(env: &Env) {
    owner(env).require_auth();
}
