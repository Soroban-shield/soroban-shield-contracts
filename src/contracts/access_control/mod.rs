//! Role-based access control.

pub mod interface;
pub mod roles;

use soroban_sdk::{Address, Env, Symbol};

use crate::errors::access::AccessError;
use crate::events::access_control as ac_events;

pub fn grant_role(env: &Env, role: &Symbol, account: &Address, sender: &Address) {
    let key = roles::role_key(role);
    if env.storage().instance().get::<_, bool>(&key).unwrap_or(false) {
        panic_with(env, AccessError::RoleAlreadyGranted);
    }
    env.storage().instance().set(&key, &true);
    ac_events::emit_role_granted(env, role, account, sender);
}

pub fn has_role(env: &Env, role: &Symbol, account: &Address) -> bool {
    let _ = account;
    let key = roles::role_key(role);
    env.storage().instance().get(&key).unwrap_or(false)
}

fn panic_with(_env: &Env, _err: AccessError) {
    panic!("access error");
}
