use cosmwasm_std::{
    attr, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult,};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{config, config_read, State};

pub const PREFIX_BALANCES: &[u8] = b"balances";
const MIN_CARD_ID_LENGTH: u64 = 12;
const MAX_CARD_ID_LENGTH: u64 = 64;
pub static CONFIG_KEY: &[u8] = b"config";
pub static MAX_UNITS: &u64= 10;
pub fn instantiate(deps: DepsMut,env: Env,info: MessageInfo,msg: InstantiateMsg,) -> Result<Response, ContractError> {
    let state = State {
        recipient: deps.api.addr_validate(&msg.recipient)?,
        source: info.sender,
        end_height: msg.end_height,
        end_time: msg.end_time,
        isvalid=true,
    };

    if state.is_expired(&env) {
        return Err(ContractError::Expired {
            end_height: msg.end_height,
            end_time: msg.end_time,
        });
    }


    config(deps.storage).save(&state)?;
    let mut total_supply: u128 = 0;
    let mut balances_store = PrefixedStorage::new(deps.storage, PREFIX_BALANCES);
    for x in msg.initial_balances {
        let amount_raw = x.amount.u128();
        balances_store.set(x.address.as_str().as_bytes(), &amount_raw.to_be_bytes());
        total_supply += amount_raw;
    }

    let config = State {
        gift_card_units: 0_u64,
    };
    save(&mut deps.storage, CONFIG_KEY, &config)?;

    Ok(Response::default())
}

// fn try_transfer_from__by_coins(deps: DepsMut,_env: Env,info: Me,owner: String,recipient: String,amount: &Uint128,) -> Result<Response, ContractError> {
//     let owner_address = deps.api.addr_validate(owner.as_str())?;
//     let recipient_address = deps.api.addr_validate(recipient.as_str())?;
//     let amount_raw = amount.u128();

//     let res = Response {
//         submessages: vec![],
//         messages: vec![],
//         attributes: vec![
//             attr("action", "transfer_from"),
//             attr("spender", &info.sender),
//             attr("sender", owner),
//             attr("recipient", recipient),
//         ],
//         data: None,
//     };
//     Ok(res)
// }

fn perform_transfer_to_giftcard(store: &mut dyn Storage,from: &Addr,to: &Addr,amount: u128, isvalid:bool,gift_id:String,) -> Result<(Response), ContractError> {
    
    let mut giftId=gift_id.as_bytes;    
    let mut config: State = load(&mut deps.storage, CONFIG_KEY)?;

    let mut from_balance = match balances_store.get(from.as_str().as_bytes()) {
        Some(data) => bytes_to_u128(&data),
        None => Ok(0u128),
    }?;

    if from_balance < amount && isvalid==true {
        return Err(ContractError::InsufficientFunds {
            balance: from_balance,
            required: amount,
        });
    }


    let stored_card = GiftCard {
        gift_balance: amount.to_vec(),
        gift_id: giftId, 
    };


    save(&mut deps.storage, &from.as_slice().to_vec(), &stored_card)?;

    config.gift_card_units += 1;
    if config.gift_card_units>MAX_UNITS {
        return Err(ContractError::MaxUnitsExceeded{
        });
    }
    save(&mut deps.storage, CONFIG_KEY, &config)?;
    from_balance -= amount;
    balances_store.set(from.as_str().as_bytes(), &from_balance.to_be_bytes());

    let res = Response {
        submessages: vec![],
        messages: vec![],
        attributes: vec![
            attr("action", "transfer_from"),
            attr("spender", &info.sender),
            attr("sender", owner),
            attr("recipient", recipient),
        ],
        data: None,
    };
    Ok(res)
}

fn validate_giftcard_id(gift_card_id: &str) -> Result<(), ContractError> {
    if (gift_card_id.len() as u64) < MIN_CARD_LENGTH {
        Err(ContractError::DescriptionTooShort {
            min_card_id_length: MIN_CARD_ID_LENGTH,
        })
    } else if (gift_card_id.len() as u64) > MAX_CARD_LENGTH {
        Err(ContractError::DescriptionTooLong {
            max_card_id_length: MAX_CARD_ID_LENGTH,
        })
    } else {
        Ok(())
    }
}

fn perform_transfer(store: &mut dyn Storage,from: &Addr,to: &Addr,amount: u128, isvalid:bool) -> Result<(), ContractError> {
    let mut balances_store = PrefixedStorage::new(store, PREFIX_BALANCES);


    let mut from_balance = match balances_store.get(from.as_str().as_bytes()) {
        Some(data) => bytes_to_u128(&data),
        None => Ok(0u128),
    }?;

    if from_balance < amount && isvalid==true {
        return Err(ContractError::InsufficientFunds {
            balance: from_balance,
            required: amount,
        });
    }
    from_balance -= amount;
    balances_store.set(from.as_str().as_bytes(), &from_balance.to_be_bytes());

    //let mut giftcard_id=match bank_read(deps.storage).may_load

    let mut giftcard_id = match balances_store.get(to.as_str().as_bytes()) {
        Some(data) => bytes_to_u128(&data).as_str(),
        None => Ok(0u128),
    }?;
    if validate_giftcard_id(giftcard_id) {
        return Err(ContractError::Invalidid{
        });
    }
    giftcard_id += amount.as_str();
    balances_store.set(to.as_str().as_bytes(), &amount.to_be_bytes());

    Ok(())
}


fn perform_transfer_to_coin(store: &mut dyn Storage,from: &Addr,to: &Addr,amount: u128, isvalid:bool) -> Result<(), ContractError> {
    let mut balances_store = PrefixedStorage::new(store, PREFIX_BALANCES);


    let mut from_balance = match balances_store.get(from.as_str().as_bytes()) {
        Some(data) => bytes_to_u128(&data),
        None => Ok(0u128),
    }?;

    if from_balance < amount && isvalid==true {
        return Err(ContractError::InsufficientFunds {
            balance: from_balance,
            required: amount,
        });
    }
    from_balance -= amount;
    balances_store.set(from.as_str().as_bytes(), &from_balance.to_be_bytes());

    //let mut giftcard_id=match bank_read(deps.storage).may_load

    let mut giftcard_id = match balances_store.get(to.as_str().as_bytes()) {
        Some(data) => bytes_to_u128(&data).as_str(),
        None => Ok(0u128),
    }?;
    if validate_giftcard_id(giftcard_id) {
        return Err(ContractError::Invalidid{
        });
    }
    giftcard_id += amount.as_str();
    balances_store.set(to.as_str().as_bytes(), &amount.to_be_bytes());

    Ok(())
}


pub fn create_key<S: Storage, A: Api, Q: Querier>(store: &mut Extern<S, A, Q>,env: Env,entropy: String,) -> StdResult<HandleResponse> {
    let constants = ReadonlyConfig::from_storage(&store.storage).constants()?;
    let prng_seed = constants.prng_seed;

    let key = ViewingKey::new(&env, &prng_seed, (&entropy).as_ref());

    let message_sender = store.api.canonical_address(&env.message.sender)?;
    write_viewing_key(&mut store.storage, &message_sender, &key);

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::CreateViewingKey { key })?),
    })
}

pub fn try_set_key<S: Storage, A: Api, Q: Querier>(store: &mut Extern<S, A, Q>,env: Env,key: String,) -> StdResult<HandleResponse> {
    let vk = ViewingKey(key);

    let message_sender = deps.api.canonical_address(&env.message.sender)?;
    write_viewing_key(&mut deps.storage, &message_sender, &vk);

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::SetViewingKey { status: Success })?),
    })
}


