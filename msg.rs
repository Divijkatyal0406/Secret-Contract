use super::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct InitialBalance {
    pub address: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct GiftCard {
    pub address: String,
    pub gift_card_id: String,
    pub gift_card_amount:Uint128,
    pub isvalid:bool,
}

impl Hashable for GiftCard {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.address.as_bytes());
        bytes.extend(&u64_bytes(&self.gift_card_id));

        bytes
    }
}


#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub initial_balances: Vec<InitialBalance>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Approve {
        spender: String,
        amount: Uint128,
    },
    Transfer {
        recipient:String,
        amount:Uint128,
    },
    TransferFromByCoin {
        recipient: Addr,
        amount:Uint128,
    },
    TransferFromByGiftCard {
        recipient: Addr,
        amount:Uint128,
        giftcardid:String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance {
        address: HumanAddr,
        key: String,
    },
    TransferHistory {
        address: HumanAddr,
        key: String,
        page: Option<u32>,
        page_size: u32,
    },
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    Balance {
        amount: Uint128,
    },
    TransferHistory {
        txs: Vec<Tx>,
    },

    ViewingKeyError {
        msg: String,
    },
}

pub fn get_validation(&self) -> (Vec<&HumanAddr>, ViewingKey) {
    match self {
        Self::TransferFromByCoin { address, key } => (vec![address], ViewingKey(key.clone())),
        Self::TransferFromByGiftCard { address, key, .. } => (vec![address], ViewingKey(key.clone())),
    }
}

pub fn space_pad(block_size: usize, message: &mut Vec<u8>) -> &mut Vec<u8> {
    let len = message.len();
    let surplus = len % block_size;
    if surplus == 0 {
        return message;
    }

    let missing = block_size - surplus;
    message.reserve(missing);
    message.extend(std::iter::repeat(b' ').take(missing));
    message
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]

