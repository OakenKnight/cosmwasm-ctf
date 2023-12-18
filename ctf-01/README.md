# Awesomwasm 2023 CTF

## Challenge 01: *Mjolnir*

Smart contract that allows user deposit for a locked period before unlocking them.

### Execute entry points:
```rust
pub enum ExecuteMsg {
    Deposit {},
    Withdraw { ids: Vec<u64> },
}
```

Please check the challenge's [integration_tests](./src/integration_tests.rs) for expected usage examples. You can use these tests as a base to create your exploit Proof of Concept.

**:house: Base scenario:**
- The contract contains initial funds.
- `USER` deposits funds into the contract.

**:star: Goal for the challenge:**
- Demonstrate how an unprivileged user can drain all funds inside the contract.


## Vulnerability 

There is a vulnerability that allows attacker to drain funds inside the contract.
Problematic part of code:
```rust
    for lockup_id in ids.clone() {
        let lockup = LOCKUPS.load(deps.storage, lockup_id).unwrap();
        lockups.push(lockup);
    }
```
If there is no checking for duplicated elements in `ids` vector, there is a possibility that attacker will pass the vector:
```rust
    vec![1,1,1,1]
```
In that case `lockups` vector will of four same `Lockup` objects, and then in the following part of code:
```rust
    for lockup in lockups {
        // validate owner and time
        if lockup.owner != info.sender || env.block.time < lockup.release_timestamp {
            return Err(ContractError::Unauthorized {});
        }

        // increase total amount
        total_amount += lockup.amount;

        // remove from storage
        LOCKUPS.remove(deps.storage, lockup.id);
    }
```
`total_amount` will be four times the value of the particular `Lockup`.

## Solution

Proposed fix is to remove duplicated elements from `ids` vector.
That involves changing mutability of passed parameter, sorting it and removing duplicated elements.

```rust
pub fn withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut ids: Vec<u64>,
) -> Result<Response, ContractError> {
    let mut lockups: Vec<Lockup> = vec![];
    let mut total_amount = Uint128::zero();

    //fix: proposed fix is to eliminate duplicated id value. 
    ids.sort();
    ids.dedup();
    ...
}
```