use soroban_sdk::{BytesN, Env, Symbol};

const IMPL: Symbol = soroban_sdk::symbol_short!("IMPL");
const LOCKED: Symbol = soroban_sdk::symbol_short!("UPLOCK");

pub fn set_implementation(env: &Env, hash: &BytesN<32>) {
    env.storage().instance().set(&IMPL, hash);
}

pub fn current_implementation(env: &Env) -> BytesN<32> {
    env.storage()
        .instance()
        .get(&IMPL)
        .expect("upgradeable: not initialized")
}

pub fn is_locked(env: &Env) -> bool {
    env.storage().instance().get(&LOCKED).unwrap_or(false)
}
