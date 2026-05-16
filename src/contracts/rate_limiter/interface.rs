pub trait RateLimiterInterface {
    fn configure(max_calls: u32, window_seconds: u64);
    fn check_and_record(caller: soroban_sdk::Address);
}
