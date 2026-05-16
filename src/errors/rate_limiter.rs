use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum RateLimiterError {
    RateLimitExceeded = 1,
    WindowTooShort = 2,
    NotConfigured = 3,
}
