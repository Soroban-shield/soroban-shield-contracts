//! Role-based access control.

pub mod contract;
pub mod interface;
pub mod roles;

#[cfg(test)]
mod test;

use soroban_sdk::{Address, Env, Symbol};

use crate::events::access_control as ac_events;

pub use contract::{revoke_role, set_role_admin};

pub fn grant_role(env: &Env, role: &Symbol, account: &Address, sender: &Address) {
    let key = roles::role_key(role);
    if env.storage().instance().get::<_, bool>(&key).unwrap_or(false) {
        panic!("role already granted");
    }
    env.storage().instance().set(&key, &true);
    ac_events::emit_role_granted(env, role, account, sender);
}

pub fn has_role(env: &Env, role: &Symbol, _account: &Address) -> bool {
    let key = roles::role_key(role);
    env.storage().instance().get(&key).unwrap_or(false)
}
