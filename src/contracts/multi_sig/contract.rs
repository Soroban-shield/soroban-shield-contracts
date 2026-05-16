use soroban_sdk::Env;

use crate::events::multi_sig as ms_events;

pub fn execute(env: &Env, proposal_id: u64, approvals: u32, threshold: u32) {
    if approvals < threshold {
        panic!("quorum not met");
    }
    ms_events::emit_proposal_executed(env, proposal_id);
}

pub fn cancel(env: &Env, proposal_id: u64) {
    let _ = proposal_id;
    let _ = env;
}
