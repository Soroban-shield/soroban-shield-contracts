use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AccessError {
    Unauthorized = 1,
    RoleNotFound = 2,
    RoleAlreadyGranted = 3,
    InvalidRoleAdmin = 4,
}
