# Course challenge: *Simplified NFT trade*

## Vulnerability

The `exec_accept_trade` function creates two submessages that are configured as `reply_always`, but the success of it is not checked in the reply entry point. If transfer fails, reply is still handled as if it was successful.

Problematic part of code for the vulnerability:

```rust
    let mut submsgs = vec![SubMsg::reply_always(
        WasmMsg::Execute {
            contract_addr: config.nft_contract.to_string(),
            msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                recipient: trade.trader.to_string(),
                token_id: trade.asked_id.clone(),
            })?,
            funds: vec![],
        },
        TRADE_REPLY,
    )];

    submsgs.push(SubMsg::reply_always(
        WasmMsg::Execute {
            contract_addr: config.nft_contract.to_string(),
            msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                recipient: sale.owner.to_string(),
                token_id: trade.to_trade_id.clone(),
            })?,
            funds: vec![],
        },
        TRADE_REPLY,
    ));
```

In addition, when a user offers a trade to an existing Sale, the contract does not require the NFT to be transferred and only validates that the trader is the current owner.

## Solution

Solution to the problem is to change `reply_always` to `reply_on_success`. Changing `reply_always` to `reply_on_success` will call `reply` endpoint only when transfer of NFT has been successful.

```rust
    let mut submsgs = vec![SubMsg::reply_on_success(
        WasmMsg::Execute {
            contract_addr: config.nft_contract.to_string(),
            msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                recipient: trade.trader.to_string(),
                token_id: trade.asked_id.clone(),
            })?,
            funds: vec![],
        },
        TRADE_REPLY,
    )];

    submsgs.push(SubMsg::reply_on_success(
        WasmMsg::Execute {
            contract_addr: config.nft_contract.to_string(),
            msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                recipient: sale.owner.to_string(),
                token_id: trade.to_trade_id.clone(),
            })?,
            funds: vec![],
        },
        TRADE_REPLY,
    ));
```
