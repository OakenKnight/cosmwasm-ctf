# Awesomwasm 2023 CTF

## Challenge 09: *Brisingamen*

Staking contract that allows the owner to distribute staking rewards for stakers.

### Execute entry points:
```rust
pub enum ExecuteMsg {
    IncreaseReward {},
    Deposit {},
    Withdraw { amount: Uint128 },
    ClaimRewards {},
}
```

Please check the challenge's [integration_tests](./src/integration_test.rs) for expected usage examples. You can use these tests as a base to create your exploit Proof of Concept.

**:house: Base scenario:**

- The contract is setup with a `USER` stake and owner has increased global index rewards.

**:star: Goal for the challenge:**

- Demonstrate how a user can earn an unfair amount of rewards in relation to other users.

## Vulnerability

Vulnerability is present because of `return` statement in `update_rewards` function.

```rust
pub fn update_rewards(user: &mut UserRewardInfo, state: &State) {
    // no need update amount if zero
    if user.staked_amount.is_zero() {
        return;
    }

    // calculate pending rewards
    let reward = (state.global_index - user.user_index) * user.staked_amount;
    user.pending_rewards += reward;

    user.user_index = state.global_index;
}
```

The bug in this implementation presents itself when existing users make a full withdrawal and then redeposit the same amount of tokens.

Because of the early return statement, their user index is not updated to the global index. If there is an increase in the global index, the function will compute rewards for the user as it thinks they have staked funds before the reward accrual periods.

Actions
| action | user |
| ------- | ------ |
| Attacker withdraws staked amount | Attacker |
| Attacker claims rewards |  Attacker|
| User 2 deposits | Victim |
| Owner increases reward | Owner |
| Attacker deposits | Attacker |
| Attacker claims rewards | Attacker |

After `attacker` claimed reward, he will receive rewards for periods he did not stake, allowing him to withdraw rewards that belong to other users.

## Solution

Solution is to remove the part about checking if `staked_amount` is zero.

```rust
pub fn update_rewards(user: &mut UserRewardInfo, state: &State) {
    // no need update amount if zero
    // if user.staked_amount.is_zero() {
    //     return;
    // }

    // calculate pending rewards
    let reward = (state.global_index - user.user_index) * user.staked_amount;
    user.pending_rewards += reward;

    user.user_index = state.global_index;
}

```

This will update the `user_index` property and by doing so prevent attacker to claim rewards belonging to other stakers.
