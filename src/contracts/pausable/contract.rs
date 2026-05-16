use soroban_sdk::Env;

use super::{is_paused, when_not_paused};

pub fn require_not_paused(env: &Env) {
    when_not_paused(env);
}

pub fn require_paused(env: &Env) {
    if !is_paused(env) {
        panic!("contract not paused");
    }
}
