//! Emergency pause circuit breaker.

pub mod contract;
pub mod interface;

use soroban_sdk::{Address, Env, Symbol};

use crate::events::pausable as pause_events;

const PAUSED: Symbol = soroban_sdk::symbol_short!("PAUSED");

pub use contract::{require_not_paused, require_paused};

pub fn pause(env: &Env, account: &Address) {
    account.require_auth();
    env.storage().instance().set(&PAUSED, &true);
    pause_events::emit_paused(env, account);
}

pub fn unpause(env: &Env, account: &Address) {
    account.require_auth();
    env.storage().instance().set(&PAUSED, &false);
    pause_events::emit_unpaused(env, account);
}

pub fn is_paused(env: &Env) -> bool {
    env.storage().instance().get(&PAUSED).unwrap_or(false)
}

pub fn when_not_paused(env: &Env) {
    if is_paused(env) {
        panic!("contract paused");
    }
}
