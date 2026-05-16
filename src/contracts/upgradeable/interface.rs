use soroban_sdk::BytesN;

pub trait UpgradeableInterface {
    fn upgrade(new_wasm_hash: BytesN<32>);
    fn current_implementation() -> BytesN<32>;
}
