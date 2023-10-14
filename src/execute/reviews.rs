use crate::{
  error::ContractError,
  msg::{ExecuteContext, ReviewInitArgs},
};
use cosmwasm_std::{attr, Response};

pub fn create(
  ctx: ExecuteContext,
  args: ReviewInitArgs,
) -> Result<Response, ContractError> {
  let ExecuteContext { deps, env, .. } = ctx;
  // TOOD: Create and save a Review in the REVIEWS map
  Ok(Response::new().add_attributes(vec![attr("action", "create_review")]))
}
