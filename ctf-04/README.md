# Awesomwasm 2023 CTF

## Challenge 04: *Gram*

Simplified vault for minting shares proportional to the current balance of the contract, it allows to redeem back for funds afterwards.
### Execute entry points:
```rust
pub enum ExecuteMsg {
    /// Mint shares
    Mint {},
    /// Burn shares
    Burn { shares: Uint128 },
}
```

Please check the challenge's [integration_tests](./src/integration_test.rs) for expected usage examples. You can use these tests as a base to create your exploit Proof of Concept.

**:house: Base scenario:**
- The contract is newly instantiated with zero funds.

**:star: Goal for the challenge:**
- Demonstrate how an unprivileged user can withdraw more funds than deposited.

## Vulnerability

Attacker can abuse mint&burn mechanism, due to poor mathematic formula used to calculate `mint_amount` and `asset_to_return`.
Actions to gain more tokens than deposited.

| Action | Attacker minted/deposited/gained token | User minted/depositet/gained token | Value in contract |Total supply minted|
| --------- | ----------- |------- | ----------- | --- |
|1. Attacker deposits 1 token | 1 | 0 | 0 | 0 |
|2. Attacker mints 1 token | 1 | 0| 1 | 1 |
|3. User deposits 20_000 tokens. | 1| 20_000| 1 | 1 |
|4. Attacker sends 10_000 tokens to contract address | 1 | 20_000 | 30_001 | 1 |
|5. User mints 20000 tokens | 1 | 20_000 | 30_001 | 2 |
|6. Attacker burns 1 token | 15_000 | 1 | 15_001 | 1 |
|7. User tries to burn 20_000 minted tokens | panic | panic| panic | panic|

In the end, attacker spent 10001 token to gain 15001 tokens back after burning, which leads to 4999 tokens profit after mint & burn abuse.

Problematic part of code in minting function:
```rust
    let mint_amount = if total_supply.is_zero() {
        amount
    } else {
        amount.multiply_ratio(total_supply, total_assets)
    };
```

Problematic part of code in burning function:
```rust
    let asset_to_return = shares.multiply_ratio(total_assets, total_supply);
```

## Solution

Solution is to change minting and burning mechanism formula.

Proposed fix for minting formula:

```rust
    let mint_amount = amount.multiply_ratio(
        total_supply + Uint128::new(10_u32.pow(DECIMAL_OFFSET).into()),
        total_assets + Uint128::one(),
    );
```
This fix heaviely relies on OpenZeppelin Solidity ERC-4626 Tokenized Vault Standard smart contract template: [link](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/f213a10522a7bd808561c5a4b17266065a199dc7/contracts/token/ERC20/extensions/ERC4626.sol#L226C1-L231C6)


Proposed fix for burning formula:
```rust
    let asset_to_return = shares.multiply_ratio(
        total_assets + Uint128::one(),
        total_supply + Uint128::new(10_u32.pow(DECIMAL_OFFSET).into()),
    );
```

This fix heaviely relies on OpenZeppelin Solidity ERC-4626 Tokenized Vault Standard smart contract template: [link](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/f213a10522a7bd808561c5a4b17266065a199dc7/contracts/token/ERC20/extensions/ERC4626.sol#L233C3-L238)
