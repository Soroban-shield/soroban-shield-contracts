use soroban_sdk::{Address, Symbol};

pub trait AccessControlInterface {
    fn grant_role(role: Symbol, account: Address);
    fn revoke_role(role: Symbol, account: Address);
    fn has_role(role: Symbol, account: Address) -> bool;
}
