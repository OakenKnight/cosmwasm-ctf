use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Selected NFT's offer is not tradeable")]
    NonTradeable {},

    #[error("The reply ID is unrecognized")]
    UnrecognizedReply {},
}
