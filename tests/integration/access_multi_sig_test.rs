use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};
use soroban_shield_contracts::contracts::{access_control, multi_sig};

#[test]
fn admin_role_can_create_proposal() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, &[] as &[u8]);
    let admin = Address::generate(&env);
    env.mock_all_auths();
    let role = symbol_short!("admin");
    let id = env.as_contract(&contract_id, || {
        access_control::grant_role(&env, &role, &admin, &admin);
        multi_sig::set_threshold(&env, 1);
        multi_sig::create_proposal(&env, &admin, symbol_short!("act"), 500)
    });
    assert_eq!(id, 1);
}
