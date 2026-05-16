use soroban_sdk::Address;

/// Ownable module public interface.
pub trait OwnableInterface {
    fn initialize(owner: Address);
    fn owner() -> Address;
    fn transfer_ownership(new_owner: Address);
    fn accept_ownership();
    fn renounce_ownership();
}
