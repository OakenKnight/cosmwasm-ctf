# Awesomwasm 2023 CTF

## Challenge 07: *Tyrfing*

Simplified vault that accounts for the top depositor! The `owner` can set the threshold to become top depositor.

### Execute entry points

```rust
pub enum ExecuteMsg {
    Deposit {},
    Withdraw { amount: Uint128 },
    OwnerAction { msg: CosmosMsg },
    UpdateConfig { new_threshold: Uint128 },
}
```

Please check the challenge's [integration_tests](./src/integration_test.rs) for expected usage examples. You can use these tests as a base to create your exploit Proof of Concept.

**:house: Base scenario:**

- The contract is newly instantiated.
- `USER1` and `USER2` deposit 100 tokens each
- The owner role is assigned to the `ADMIN` address
- Threshold is set to 99 when instantiating contract

**:star: Goal for the challenge:**

- Demonstrate how an unprivileged user can drain all the contract's funds.

## Vulnerability

Vulnerability lies in using the same storage key for both `TOP_DEPOSITOR` and `OWNER` states. By becoming the `TOP_DEPOSITOR` malicious user can become the `OWNER` and thus act maliciously.
`TOP_DEPOSITOR` state from `contract.rs`.

```rust
pub const TOP_DEPOSITOR: Item<Addr> = Item::new("address");
```

`OWNER` state from `contract.rs`.

```rust
pub const OWNER: Item<Addr> = Item::new("address");
```

## Solution

Solution is to change the storage key for `TOP_DEPOSITOR`.

```rust
pub const TOP_DEPOSITOR: Item<Addr> = Item::new("top_depositor");
```