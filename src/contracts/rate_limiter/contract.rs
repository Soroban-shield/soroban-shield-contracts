use soroban_sdk::{Address, Env, Symbol};

pub fn reset(env: &Env, caller: &Address) {
    let key = (Symbol::new(env, "rl"), caller.clone());
    env.storage().instance().remove(&key);
}
