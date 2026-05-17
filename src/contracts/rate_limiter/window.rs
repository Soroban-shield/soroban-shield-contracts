pub fn within_window(now: u64, start: u64, window_secs: u64) -> bool {
    now.saturating_sub(start) < window_secs
}
