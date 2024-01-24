# Challenge: *Simplified NFT trade*

Open marketplace for an NFT project. Users can sell their own NFTs at any price or allow others to offer different NFTs in exchange to trade.

## Execute entry points

```rust
pub enum ExecuteMsg {
    NewSale {
        id: String,
        price: Uint128,
    },
    NewTrade {
        target: String,
        offered: String,
    },
    AcceptTrade {
        id: String,
        trader: String,
    },
}
```

Please check the challenge's [integration_tests](/src/integration_tests.rs) for expected usage examples. You can use these tests as a base to create your exploit Proof of Concept.

**:house: Base scenario:**

- The contract is newly instantiated.
- `USER1` and `USER2` placed new sales of their NFTs, one of them is open for trades and the other does not.

**:star: Goal for the challenge:**

- Demonstrate how a user can retrieve other users' NFT for free.
