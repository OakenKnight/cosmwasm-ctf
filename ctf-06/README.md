# Awesomwasm 2023 CTF

## Challenge 06: *Hofund*

The contract allow anyone to propose themselves for the `owner` role of the contract, the rest of the users can vote in favor by sending a governance token. If a proposal was voted for with more than a third of the current supply, the user gets the `owner` role.

### Execute entry points:
```rust
pub enum ExecuteMsg {
    Propose {},
    ResolveProposal {},
    OwnerAction {
        action: CosmosMsg,
    },
    Receive(Cw20ReceiveMsg),
}
```

Please check the challenge's [integration_tests](./src/integration_test.rs) for expected usage examples. You can use these tests as a base to create your exploit Proof of Concept.

**:house: Base scenario:**
- The contract is newly instantiated

**:star: Goal for the challenge:**
- Demonstrate how a proposer can obtain the owner role without controlling 1/3 of the total supply.

## Vulnerability

Vulnerability presents itself because the contract checks for contract `balance`. That implies the following scenario.

```rust
    if balance.balance >= (vtoken_info.total_supply / Uint128::from(3u32)) {
        CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
            config.owner = current_proposal.proposer;
            Ok(config)
        })?;
        response = response.add_attribute("result", "Passed");
    } else {
        PROPOSAL.remove(deps.storage);
        response = response.add_attribute("result", "Failed");
    }
```

| action | total tokens | User1 | Attacker | contract balance| 
|--------| ------------ | ----- | ----- | -------- |
| / |  12 | 3 | 1 | 0 |
| User1 proposes | 12 | 0 | 1 | 3 |
| User1 proposes end - *FAIL* | 12 | 0 | 1 | 3 |
| Attacker proposes| 12 | 0 | 0 | 4|
| Attacker proposes end - *SUCCESS* | 12 | 0 | 0 | 4 |

After User1 proposed the end, the assets didnt return to him, but rather stayed on the contract. After that, attacker is able to propose another challenge with 1 token, and win the ownership.

Also there is a problem with matching in `receive_cw20` function. If any message, except for `CastVote` is sent, `match` will just return `Ok(Response::default())`.

## Solution

Solution is to track how much tokens is voted with on proposal. That data needs to be saved in state and used later when checking if proposer has won or not.

```rust
 if current_proposal.voted_with >= (vtoken_info.total_supply / Uint128::from(3u32)) {
        CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
            config.owner = current_proposal.proposer;
            Ok(config)
        })?;
        response = response.add_attribute("result", "Passed");
    } else {
        PROPOSAL.remove(deps.storage);
        response = response.add_attribute("result", "Failed");
    }
```

Now, about other problem regarding matching. Match function should return `Err(ContractError::Unauthorized {})` instead of `Ok(Response::default())`.
