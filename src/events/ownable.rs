use soroban_sdk::{Address, Env, Symbol};

pub fn emit_ownership_transfer_started(env: &Env, previous: &Address, pending: &Address) {
    let topics = (Symbol::new(env, "ownership_started"), previous, pending);
    env.events().publish(topics, ());
}

pub fn emit_ownership_transferred(env: &Env, previous: &Address, new_owner: &Address) {
    let topics = (Symbol::new(env, "ownership_xfer"), previous, new_owner);
    env.events().publish(topics, ());
}
