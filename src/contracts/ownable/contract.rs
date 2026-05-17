use soroban_sdk::{Address, Env};

use super::{only_owner, owner, OWNER, PENDING};
use crate::events::ownable as ownable_events;

pub fn transfer_ownership(env: &Env, new_owner: &Address) {
    only_owner(env);
    let current = owner(env);
    env.storage().instance().set(&PENDING, new_owner);
    ownable_events::emit_ownership_transfer_started(env, &current, new_owner);
}

pub fn accept_ownership(env: &Env) {
    let pending: Address = env
        .storage()
        .instance()
        .get(&PENDING)
        .expect("ownable: no pending owner");
    pending.require_auth();
    let previous = owner(env);
    env.storage().instance().set(&OWNER, &pending);
    env.storage().instance().remove(&PENDING);
    ownable_events::emit_ownership_transferred(env, &previous, &pending);
}

pub fn renounce_ownership(env: &Env) {
    only_owner(env);
    env.storage().instance().remove(&OWNER);
    env.storage().instance().remove(&PENDING);
}
