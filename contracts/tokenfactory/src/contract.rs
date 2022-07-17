use std::convert::TryInto;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{CosmosMsg, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::osmo_msg::MsgCreateDenom;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:tokenfactory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateDenom { subdenom } => try_create_denom(env, subdenom),
    }
}

pub fn try_create_denom(env: Env, subdenom: String) -> Result<Response, ContractError> {
    let msg_create_denom: CosmosMsg = MsgCreateDenom {
        sender: env.contract.address.into(),
        subdenom,
    }
    .try_into()
    .map_err(ContractError::Std)?;

    Ok(Response::new()
        .add_message(msg_create_denom)
        .add_attribute("method", "try_create_denom"))
}
