use soroban_sdk::{Address, Env, Symbol};

pub fn emit_proposal_created(env: &Env, proposal_id: u64, creator: &Address) {
    let topics = (Symbol::new(env, "proposal_created"), proposal_id, creator);
    env.events().publish(topics, ());
}

pub fn emit_proposal_executed(env: &Env, proposal_id: u64) {
    let topics = (Symbol::new(env, "proposal_executed"), proposal_id);
    env.events().publish(topics, ());
}
