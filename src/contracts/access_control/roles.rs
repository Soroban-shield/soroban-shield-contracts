use soroban_sdk::Symbol;

pub fn role_key(role: &Symbol) -> (Symbol, Symbol) {
    (soroban_sdk::symbol_short!("ROLE"), role.clone())
}

pub fn admin_key(role: &Symbol) -> (Symbol, Symbol) {
    (soroban_sdk::symbol_short!("RADMIN"), role.clone())
}
