use soroban_sdk::{Address, Env, Symbol};

use super::roles;
use crate::events::access_control as ac_events;

pub fn revoke_role(env: &Env, role: &Symbol, account: &Address, sender: &Address) {
    let key = roles::role_key(role);
    env.storage().instance().remove(&key);
    ac_events::emit_role_revoked(env, role, account, sender);
}

pub fn set_role_admin(env: &Env, role: &Symbol, admin_role: &Symbol) {
    let key = roles::admin_key(role);
    env.storage().instance().set(&key, admin_role);
}
