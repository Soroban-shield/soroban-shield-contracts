use soroban_sdk::{contracttype, BytesN, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub id: u64,
    pub action: Symbol,
    pub payload_hash: BytesN<32>,
    pub expiry_ledger: u32,
    pub approvals: u32,
    pub executed: bool,
}
