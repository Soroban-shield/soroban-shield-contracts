use soroban_sdk::{Address, Env, Symbol};

pub fn emit_paused(env: &Env, account: &Address) {
    let topics = (Symbol::new(env, "paused"), account);
    env.events().publish(topics, ());
}

pub fn emit_unpaused(env: &Env, account: &Address) {
    let topics = (Symbol::new(env, "unpaused"), account);
    env.events().publish(topics, ());
}
