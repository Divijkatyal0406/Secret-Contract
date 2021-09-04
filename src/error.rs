use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Transaction expired")]
    Expired {
        end_height: Option<u64>,
        end_time: Option<u64>,
    },

    #[error("card id too short (minimum id length {min_desc_length})")]
    CardIdTooShort { min_desc_length: u64 },

    #[error("maximum units allocated for this gift card are over")]
    MaxUnitsExceeded {},

    #[error("id too long (maximum id length {max_desc_length})")]
    CardIDTooLong { max_desc_length: u64 },

    #[error("Insufficient funds")]
    InsufficientFunds { balance: u128, required: u128 },

}
