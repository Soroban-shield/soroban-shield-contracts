use soroban_sdk::{Address, Env, Symbol};

pub fn emit_role_granted(env: &Env, role: &Symbol, account: &Address, sender: &Address) {
    let topics = (Symbol::new(env, "role_granted"), role, account, sender);
    env.events().publish(topics, ());
}

pub fn emit_role_revoked(env: &Env, role: &Symbol, account: &Address, sender: &Address) {
    let topics = (Symbol::new(env, "role_revoked"), role, account, sender);
    env.events().publish(topics, ());
}
