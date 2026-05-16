use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MultiSigError {
    QuorumNotMet = 1,
    ProposalNotFound = 2,
    ProposalExpired = 3,
    ProposalAlreadyExecuted = 4,
    DuplicateApproval = 5,
}
