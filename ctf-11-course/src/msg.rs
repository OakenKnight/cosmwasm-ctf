use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub nft_address: String,
}

#[cw_serde]
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

#[cw_serde]
pub enum QueryMsg {
    GetSale {
        id: String,
    },
    GetTrade {
        id: String,
        trader: String,
    },

}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
