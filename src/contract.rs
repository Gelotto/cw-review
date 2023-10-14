use crate::error::ContractError;
use crate::execute;
use crate::msg::{
  ExecuteContext, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryContext, QueryMsg, ReviewExecuteMsg,
  ReviewQueryMsg,
};
use crate::query;
use crate::state;
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-contract-template";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  state::initialize(deps, &env, &info, &msg)?;
  Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  let ctx = ExecuteContext { deps, env, info };
  match msg {
    ExecuteMsg::Reviews(msg) => match msg {
      ReviewExecuteMsg::Create(args) => execute::reviews::create(ctx, args),
    },
  }
}

#[entry_point]
pub fn query(
  deps: Deps,
  env: Env,
  msg: QueryMsg,
) -> Result<Binary, ContractError> {
  let ctx = QueryContext { deps, env };
  let result = match msg {
    QueryMsg::Reviews(msg) => match msg {
      ReviewQueryMsg::Get { id } => to_binary(&query::reviews::get(ctx, id)?),
    },
  }?;
  Ok(result)
}

#[entry_point]
pub fn migrate(
  _deps: DepsMut,
  _env: Env,
  _msg: MigrateMsg,
) -> Result<Response, ContractError> {
  Ok(Response::default())
}
