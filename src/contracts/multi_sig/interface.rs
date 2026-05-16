pub trait MultiSigInterface {
    fn create_proposal(action: soroban_sdk::Symbol, expiry_ledger: u32) -> u64;
    fn approve(proposal_id: u64);
    fn execute(proposal_id: u64);
}
