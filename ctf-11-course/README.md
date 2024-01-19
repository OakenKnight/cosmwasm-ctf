# Awesomwasm 2023 CTF

## Challenge 08: *Gjallarhorn*

Open marketplace for an NFT project. Users can sell their own NFTs at any price or allow others to offer different NFTs in exchange to trade.

### Execute entry points:
```rust
pub enum ExecuteMsg {
    BuyNFT {
        id: String,
    },
    NewSale {
        id: String,
        price: Uint128,
        tradable: bool,
    },
    CancelSale {
        id: String,
    },
    NewTrade {
        target: String,
        offered: String,
    },
    AcceptTrade {
        id: String,
        trader: String,
    },
    CancelTrade {
        id: String,
    },
}
```

Please check the challenge's [integration_tests](./src/integration_test.rs) for expected usage examples. You can use these tests as a base to create your exploit Proof of Concept.

**:house: Base scenario:**
- The contract is newly instantiated.
- `USER1` and `USER2` placed new sales of their NFTs, one of them is open for trades and the other does not.

**:star: Goal for the challenge:**
- Demonstrate how a user can retrieve other users' NFT for free.

## Vulnerability

There are two vulnerabilities.

1. Not removing sale from the state after successfull trade was completed
2. There is no checking success of it is not checked in the reply entry point.


Problematic part of code for second vulnerability:

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
```
And
```rust
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

Solution to the first problem is to remove sale from storage if trade was accepted.
```rust
    SALES.remove(deps.storage, trade.asked_id.clone());
```

Solution to the second problem is to change `reply_always` to `reply_on_success`.