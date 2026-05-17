use soroban_sdk::{Address, Env};

/// Returns the authenticated caller address.
pub fn caller(env: &Env) -> Address {
    env.current_contract_address()
}

/// Require that `account` has authorized the current call.
pub fn require_auth(_env: &Env, account: &Address) {
    account.require_auth();
}
