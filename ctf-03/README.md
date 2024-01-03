# Oak Security CosmWasm CTF

## Challenge 03: *Laevateinn*

Flash loan protocol which allows users to execute a [flash loan](https://chain.link/education-hub/flash-loans) using the proxy contract.

### Flash loan contract entry points:
```rust
pub enum ExecuteMsg {
    SetProxyAddr { proxy_addr: String },
    FlashLoan {},
    SettleLoan {},
    WithdrawFunds { recipient: Addr },
    TransferOwner { new_owner: Addr },
}
```

### Proxy contract entry points:
```rust
pub enum ExecuteMsg {
    RequestFlashLoan { recipient: Addr, msg: Binary },
}
```

Please check the challenge's [integration_tests](./src/integration_test.rs) for expected usage examples. You can use these tests as a base to create your exploit Proof of Concept.

**:house: Base scenario:**
- The flash loan contract will have initial funds deposited.
- Proxy contract is configured to flash loan contract.

**:star: Goal for the challenge:**
- Demonstrate how an unprivileged user can drain all funds from the flash loan contract.

## Vulnerability

When requesting a flash loan, the `CallToFlashLoan` error will revert the transaction if the recipient address is the flash loan contract address. This validation is important to prevent the user from controlling the proxy contract to execute authenticated messages in the flash loan contract, such as the `TransferOwner` message. Example of not validation the address can be found in `request_flash_loan` function.

```rust 
   if recipient == config.flash_loan_addr {
        return Err(ContractError::CallToFlashLoan {});
    }
```

If recipient is not in the same format (all lowercase), attacker is able to bypass the validation and control the proxy contract to execute authenticated messages in the flash loan contract. After that, it is easy to execute `TransferOwner` message with the new_owner address set to our address. Once we obtain ownership of the flash loan contract, we call the `WithdrawFunds` message to complete our exploit flow.

## Solution

Solution is rather simple. When instantiateing the contract, the flash loan contract address is validated with the `addr_validate` function, and by doing that, we are essentially just lowercasing the address, so having that in mind, we should also lowercase `recepient` address in `request_flash_loan` function.

```rust
    let recipient = deps.api.addr_validate(&recipient.to_string())?;
    // Disallow calling flash loan addr
    if recipient == config.flash_loan_addr {
        return Err(ContractError::CallToFlashLoan {});
    }
```