//! N-of-M threshold execution for privileged operations.

pub mod contract;
pub mod interface;
pub mod proposal;

#[cfg(test)]
mod test;

use soroban_sdk::{Address, Env, Symbol};

use crate::events::multi_sig as ms_events;

const THRESHOLD: Symbol = soroban_sdk::symbol_short!("MSTHR");
const NEXT_ID: Symbol = soroban_sdk::symbol_short!("MSID");

pub use contract::{cancel, execute};

pub fn set_threshold(env: &Env, threshold: u32) {
    env.storage().instance().set(&THRESHOLD, &threshold);
}

pub fn create_proposal(env: &Env, creator: &Address, action: Symbol, expiry_ledger: u32) -> u64 {
    creator.require_auth();
    let id: u64 = env.storage().instance().get(&NEXT_ID).unwrap_or(1);
    env.storage().instance().set(&NEXT_ID, &(id + 1));
    ms_events::emit_proposal_created(env, id, creator);
    let _ = (action, expiry_ledger);
    id
}
