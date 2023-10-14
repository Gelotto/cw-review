use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Deps, DepsMut, Env, MessageInfo};

use crate::models::ReviewCategory;

pub struct ExecuteContext<'a> {
  pub deps: DepsMut<'a>,
  pub env: Env,
  pub info: MessageInfo,
}

pub struct QueryContext<'a> {
  pub deps: Deps<'a>,
  pub env: Env,
}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
  Reviews(ReviewExecuteMsg),
}

#[cw_serde]
pub enum ReviewExecuteMsg {
  Create(ReviewInitArgs),
}

#[cw_serde]
pub enum QueryMsg {
  Reviews(ReviewQueryMsg),
}

#[cw_serde]
pub enum ReviewQueryMsg {
  Get { id: String },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ReviewInitArgs {
  pub address: Addr,
  pub category: ReviewCategory,
  pub comment: Option<String>,
  pub rating: Option<u8>,
  pub evidence: Option<Vec<EvidenceItemInitArgs>>,
}

#[cw_serde]
pub struct EvidenceItemInitArgs {
  pub description: String,
  pub image_url: Option<String>,
  pub url: Option<String>,
}
