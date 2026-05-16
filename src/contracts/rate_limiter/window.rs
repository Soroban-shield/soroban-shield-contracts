use soroban_sdk::{Address, Env};

#[derive(Clone)]
pub struct WindowState {
    pub count: u32,
    pub window_start: u64,
}

pub fn within_window(now: u64, start: u64, window_secs: u64) -> bool {
    now.saturating_sub(start) < window_secs
}
