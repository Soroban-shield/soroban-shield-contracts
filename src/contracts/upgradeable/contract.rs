use soroban_sdk::{BytesN, Env};

use super::proxy;

pub fn initialize_implementation(env: &Env, hash: &BytesN<32>) {
    proxy::set_implementation(env, hash);
}
