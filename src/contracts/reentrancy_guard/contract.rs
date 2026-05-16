use soroban_sdk::Env;

use super::{enter, exit};

pub fn non_reentrant<F>(env: &Env, f: F)
where
    F: FnOnce(&Env),
{
    enter(env);
    f(env);
    exit(env);
}
