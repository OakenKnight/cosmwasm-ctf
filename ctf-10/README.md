# Awesomwasm 2023 CTF

## Challenge 10: *Mistilteinn*

Smart contract that allows whitelisted users to mint NFTs.

### Execute entry points:
```rust
pub enum ExecuteMsg {
    Mint {},
}
```

Please check the challenge's [integration_tests](./src/integration_test.rs) for expected usage examples. You can use these tests as a base to create your exploit Proof of Concept.

**:house: Base scenario:**
- The contract is instantiated with whitelisted users as `USER1`, `USER2`, and `USER3`.

**:star: Goal for the challenge:**
- Demonstrate how whitelisted users can bypass the `mint_per_user` limitation.

## Vulnerability

Vulnerability presents itself when checking for minted tokens by user. Checking if user is able to mint token is done by tracking the number of tokens owned by user. If tokens are transfered to other accounts, that account will be able to mint more tokens and send them to not whitelisted addresses.

Problematic part of code:
```rust
    let tokens_response: TokensResponse = deps.querier.query_wasm_smart(
        config.nft_contract.to_string(),
        &Cw721QueryMsg::Tokens::<Empty> {
            owner: info.sender.to_string(),
            start_after: None,
            limit: None,
        },
    )?;

    // ensure mint per user limit is not exceeded
    if tokens_response.tokens.len() >= config.mint_per_user as usize {
        return Err(ContractError::MaxLimitExceeded {});
    }
```


## Solution

Solution is not to track number of currently owned tokens by user, but rather to store the tokens in contract state in a map:

```rust
#[cw_serde]
    pub struct MintedNFT {
        /// whitelisted users to receive NFTs
        pub nft_id: u128,
        pub timestamp: u64
    }
    pub const MINT_PER_USER: Map<&Addr, Vec<MintedNFT>> = Map::new("mint_per_user");
```

After every minting of token, map needs to be updated.
```rust
    let mut minted_nfts = MINT_PER_USER
        .load(deps.storage, &info.sender)
        .unwrap_or_default();
    let nft_to_mint : MintedNFT = MintedNFT {
        nft_id : token_id,
        timestamp : env.block.time.seconds(),
    };

    minted_nfts.push(nft_to_mint);

    MINT_PER_USER.save(deps.storage, &info.sender, &minted_nfts)?;
```

New condition to mint token should look like this: 

```rust
    let minted_nfts: Vec<MintedNFT> = deps.querier.query_wasm_smart(
        env.contract.address.to_string(),
        &QueryMsg::MintPerUser {
            user: info.sender.to_string(),
            limit: None,
        },
    )?;

    // ensure mint per user limit is not exceeded
    if minted_nfts.len() >= config.mint_per_user as usize {
        return Err(ContractError::MaxLimitExceeded {});
    }
```