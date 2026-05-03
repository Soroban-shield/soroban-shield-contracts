# soroban-shield-contracts

> Core Rust library of audited, modular Soroban smart contract components for the Stellar ecosystem.

[![Stellar Wave](https://img.shields.io/badge/Stellar%20Wave-Wave%205-blue?style=flat-square)](https://www.drips.network/wave/stellar)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=flat-square)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow?style=flat-square)](LICENSE)
[![CI](https://img.shields.io/badge/CI-GitHub%20Actions-green?style=flat-square)](.github/workflows/ci.yml)

---

## Overview

`soroban-shield-contracts` is the heart of the Soroban Shield organization. It provides a collection of secure, reusable Rust smart contract modules deployable on the Stellar blockchain via Soroban. Each module is self-contained, well-documented, and designed to be composed with others.

---

## File Structure

```
soroban-shield-contracts/
│
├── Cargo.toml                          # Workspace manifest
├── Cargo.lock
├── README.md                           # This file
├── CONTRIBUTING.md                     # Contribution guidelines (mirrors org root)
├── LICENSE
├── CODEOWNERS
├── .gitignore
│
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                      # Build, test, clippy, fmt on every PR
│   │   └── release.yml                 # Publish crate on tag push
│   └── ISSUE_TEMPLATE/
│       ├── bug_report.md
│       ├── feature_request.md
│       └── stellar_wave_task.md        # Template for Wave-scoped issues
│
├── src/
│   ├── lib.rs                          # Crate root — re-exports all modules
│   │
│   ├── contracts/
│   │   ├── mod.rs
│   │   │
│   │   ├── ownable/
│   │   │   ├── mod.rs                  # Ownable contract: single-owner with 2-step transfer
│   │   │   ├── contract.rs             # #[contract] impl
│   │   │   ├── interface.rs            # Trait definition
│   │   │   └── test.rs                 # Unit tests
│   │   │
│   │   ├── access_control/
│   │   │   ├── mod.rs                  # Role-based access control (RBAC)
│   │   │   ├── contract.rs
│   │   │   ├── interface.rs
│   │   │   ├── roles.rs                # Role constants and helpers
│   │   │   └── test.rs
│   │   │
│   │   ├── pausable/
│   │   │   ├── mod.rs                  # Emergency pause / unpause with role gate
│   │   │   ├── contract.rs
│   │   │   ├── interface.rs
│   │   │   └── test.rs
│   │   │
│   │   ├── reentrancy_guard/
│   │   │   ├── mod.rs                  # Cross-contract reentrancy protection
│   │   │   ├── contract.rs
│   │   │   ├── interface.rs
│   │   │   └── test.rs
│   │   │
│   │   ├── rate_limiter/
│   │   │   ├── mod.rs                  # Per-address call throttling with sliding window
│   │   │   ├── contract.rs
│   │   │   ├── interface.rs
│   │   │   ├── window.rs               # Sliding window logic
│   │   │   └── test.rs
│   │   │
│   │   ├── multi_sig/
│   │   │   ├── mod.rs                  # N-of-M signature threshold execution
│   │   │   ├── contract.rs
│   │   │   ├── interface.rs
│   │   │   ├── proposal.rs             # Proposal lifecycle types
│   │   │   └── test.rs
│   │   │
│   │   └── upgradeable/
│   │       ├── mod.rs                  # WASM proxy upgrade pattern for Soroban
│   │       ├── contract.rs
│   │       ├── interface.rs
│   │       ├── proxy.rs                # Proxy delegation logic
│   │       └── test.rs
│   │
│   ├── types/
│   │   ├── mod.rs
│   │   ├── address.rs                  # Shared address wrapper types
│   │   ├── role.rs                     # Generic role type definitions
│   │   └── proposal.rs                 # Shared proposal types for multi-sig
│   │
│   ├── events/
│   │   ├── mod.rs
│   │   ├── ownable.rs                  # OwnershipTransferred, OwnershipTransferStarted
│   │   ├── access_control.rs           # RoleGranted, RoleRevoked
│   │   ├── pausable.rs                 # Paused, Unpaused
│   │   └── multi_sig.rs               # ProposalCreated, ProposalExecuted, ProposalCancelled
│   │
│   └── errors/
│       ├── mod.rs
│       ├── access.rs                   # Unauthorized, RoleNotFound, etc.
│       ├── pausable.rs                 # ContractPaused, ContractNotPaused
│       ├── rate_limiter.rs             # RateLimitExceeded, WindowTooShort
│       └── multi_sig.rs               # QuorumNotMet, ProposalExpired, etc.
│
├── tests/
│   ├── unit/
│   │   ├── ownable_test.rs
│   │   ├── access_control_test.rs
│   │   ├── pausable_test.rs
│   │   ├── reentrancy_guard_test.rs
│   │   ├── rate_limiter_test.rs
│   │   ├── multi_sig_test.rs
│   │   └── upgradeable_test.rs
│   │
│   └── integration/
│       ├── ownable_pausable_test.rs    # Composability: Ownable + Pausable
│       ├── access_multi_sig_test.rs   # Composability: AccessControl + MultiSig
│       └── full_stack_test.rs         # All modules composed together
│
└── scripts/
    ├── build.sh                        # cargo build --target wasm32-unknown-unknown
    ├── test.sh                         # cargo test + fmt + clippy
    └── deploy_testnet.sh               # soroban contract deploy to testnet
```

---

## Modules

### `Ownable`
Single-owner contract with a secure two-step ownership transfer. The pending owner must explicitly accept before ownership changes.

**Key functions:** `initialize(owner)`, `transfer_ownership(new_owner)`, `accept_ownership()`, `renounce_ownership()`, `owner()`

### `AccessControl`
Role-based access control. Roles are `Symbol`-keyed and can be granted, revoked, and queried. Supports role admins.

**Key functions:** `grant_role(role, account)`, `revoke_role(role, account)`, `has_role(role, account)`, `set_role_admin(role, admin_role)`

### `Pausable`
Emergency circuit breaker. When paused, guarded functions revert. Integrates with `AccessControl` for role-gated pause.

**Key functions:** `pause()`, `unpause()`, `is_paused()`

### `ReentrancyGuard`
Prevents reentrant calls within a single contract execution. Uses Soroban ledger storage as the lock mechanism.

**Key functions:** `enter()`, `exit()` — typically used as guards wrapping function bodies.

### `RateLimiter`
Per-address call throttling using a configurable sliding window. Useful for preventing spam or abuse.

**Key functions:** `configure(max_calls, window_seconds)`, `check_and_record(caller)`, `reset(caller)`

### `MultiSig`
N-of-M signature threshold for executing privileged operations. Proposals expire after a configurable TTL.

**Key functions:** `create_proposal(action, expiry)`, `approve(proposal_id)`, `execute(proposal_id)`, `cancel(proposal_id)`

### `Upgradeable`
WASM proxy upgrade pattern compatible with Soroban's contract deployment model. Stores the current implementation hash and delegates calls.

**Key functions:** `upgrade(new_wasm_hash)`, `current_implementation()`, `lock_upgrades()`

---

## Getting Started

### Prerequisites

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Soroban CLI
cargo install --locked soroban-cli
```

### Build

```bash
git clone https://github.com/soroban-shield/soroban-shield-contracts
cd soroban-shield-contracts
cargo build --target wasm32-unknown-unknown --release
```

### Test

```bash
cargo test
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

### Use a Module in Your Project

Add to your `Cargo.toml`:

```toml
[dependencies]
soroban-shield-contracts = { git = "https://github.com/soroban-shield/soroban-shield-contracts", tag = "v0.1.0" }
```

Example — adding `Ownable` to your contract:

```rust
use soroban_shield_contracts::contracts::ownable::interface::OwnableInterface;

#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn sensitive_action(env: Env) {
        // Restricts to owner only
        ownable::only_owner(&env);
        // ... your logic
    }
}
```

---

## Stellar Wave — Open Issues

This repository participates in the **Stellar Wave Program**. Issues tagged `Stellar Wave` are available for community contributors to apply for and earn USDC rewards.

Browse open Wave issues: [github.com/soroban-shield/soroban-shield-contracts/issues](https://github.com/soroban-shield/soroban-shield-contracts/issues?q=label%3A%22Stellar+Wave%22)

To contribute via Wave:
1. Sign in at [drips.network/wave/stellar](https://www.drips.network/wave/stellar)
2. Find an issue in this repo and click **Apply**
3. Wait for assignment — do not start before being assigned
4. Submit a PR referencing the issue

**Points:** Trivial = 100 pts | Medium = 150 pts | High = 200 pts

---

## License

MIT — see [LICENSE](LICENSE)
