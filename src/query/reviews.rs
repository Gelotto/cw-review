use crate::{error::ContractError, models::Review, msg::QueryContext};

/// Get and return a Review by ID
pub fn get(
  ctx: QueryContext,
  review_id: String,
) -> Result<Review, ContractError> {
  Review::load_by_id(ctx.deps.storage, &review_id)
}
