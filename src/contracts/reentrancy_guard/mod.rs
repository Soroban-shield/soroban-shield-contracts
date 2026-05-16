//! Cross-call reentrancy protection via instance lock.

pub mod contract;
pub mod interface;

#[cfg(test)]
mod test;

use soroban_sdk::{Env, Symbol};

const LOCK: Symbol = soroban_sdk::symbol_short!("REENT");

pub use contract::non_reentrant;

pub fn enter(env: &Env) {
    if env.storage().instance().get(&LOCK).unwrap_or(false) {
        panic!("reentrancy guard: reentrant call");
    }
    env.storage().instance().set(&LOCK, &true);
}

pub fn exit(env: &Env) {
    env.storage().instance().set(&LOCK, &false);
}
