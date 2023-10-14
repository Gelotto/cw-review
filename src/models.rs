use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Storage, Timestamp};

use crate::{error::ContractError, state::REVIEWS};

#[cw_serde]
pub enum ReviewCategory {
  Feedback,
  Hack,
  Scam,
  Spam,
}

#[cw_serde]
pub struct Review {
  pub id: Option<String>,
  pub category: ReviewCategory,
  pub created_at: Timestamp,
  pub updated_at: Option<Timestamp>,
  pub comment: Option<String>,
  pub rating: Option<u8>,
  pub evidence: Vec<EvidenceItem>,
}

#[cw_serde]
pub struct EvidenceItem {
  pub description: String,
  pub image_url: Option<String>,
  pub url: Option<String>,
  pub likes: u32,
}

impl Review {
  pub fn load_by_id(
    storage: &dyn Storage,
    id: &String,
  ) -> Result<Self, ContractError> {
    REVIEWS
      .load(storage, id)
      .map_err(|_| ContractError::NotFound {
        reason: Some(format!("review {} not found", id)),
      })
  }
}
