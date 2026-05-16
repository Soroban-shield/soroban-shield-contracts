//! Per-address call throttling with sliding window.

pub mod interface;
pub mod window;

use soroban_sdk::{Address, Env, Symbol};

use crate::errors::rate_limiter::RateLimiterError;

const MAX: Symbol = soroban_sdk::symbol_short!("RLMAX");
const WIN: Symbol = soroban_sdk::symbol_short!("RLWIN");

pub fn configure(env: &Env, max_calls: u32, window_seconds: u64) {
    if window_seconds == 0 {
        panic!("window too short");
    }
    env.storage().instance().set(&MAX, &max_calls);
    env.storage().instance().set(&WIN, &window_seconds);
}

pub fn check_and_record(env: &Env, caller: &Address, now: u64) -> Result<(), RateLimiterError> {
    let max: u32 = env.storage().instance().get(&MAX).ok_or(RateLimiterError::NotConfigured)?;
    let window: u64 = env.storage().instance().get(&WIN).ok_or(RateLimiterError::NotConfigured)?;
    let key = (Symbol::new(env, "rl"), caller.clone());
    let mut state: (u32, u64) = env.storage().instance().get(&key).unwrap_or((0, now));
    if !window::within_window(now, state.1, window) {
        state = (0, now);
    }
    if state.0 >= max {
        return Err(RateLimiterError::RateLimitExceeded);
    }
    state.0 += 1;
    env.storage().instance().set(&key, &state);
    Ok(())
}
