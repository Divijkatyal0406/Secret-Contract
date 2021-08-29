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
        recipient
    },
    TransferFromByGiftCard {
        recipient
    },
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

