# Awesomwasm 2023 CTF

## Challenge 02: *Gungnir*

Staking contract for users to lock their deposits for a fixed amount of time to generate voting power.

### Execute entry points:
```rust
pub enum ExecuteMsg {
    Deposit {},
    Withdraw { amount: Uint128 },
    Stake { lock_amount: u128 },
    Unstake { unlock_amount: u128 },
}
```

Please check the challenge's [integration_tests](./src/integration_test.rs) for expected usage examples. You can use these tests as a base to create your exploit Proof of Concept.

**:house: Base scenario:**
- The contract is newly instantiated with zero funds.

**:star: Goal for the challenge:**
- Demonstrate how an unprivileged user can achieve an unfair amount of voting power.

## Vulnerability 

Vulnerability presents itself in ability for user to get extremely large amount of voting power by abusing overflow.
Problematic part of code:

```rust
    user.voting_power -= unlock_amount;
```

Main cause of the vulnerability is having configuration parameter `overflow-checks` was set to `false` in `Cargo.toml`. This will cause overflow when executing `unstake` because `voting_power` is of type `u128`.
Problem will not appear when debugging test and unstaking more then staked will cause `panic`, but in `release` mode, it will cause overflow.

## Solution

```rust
overflow-checks = true
```
