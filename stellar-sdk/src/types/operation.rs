use serde::{Deserialize, Serialize};

use crate::endpoints::horizon::{Claimant, Path, PriceRShortHand, Reserve, ResponseLink};

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationLinks {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub itself: ResponseLink,
    pub transaction: ResponseLink,
    pub effects: ResponseLink,
    pub succeeds: ResponseLink,
    pub precedes: ResponseLink,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operation {
    pub _links: OperationLinks,
    pub id: String,
    pub paging_token: String,
    pub transaction_successful: bool,
    pub source_account: String,
    pub r#type: String,
    pub type_i: u32,
    pub created_at: String,
    pub transaction_hash: String,
    pub starting_balance: Option<String>,
    pub funder: Option<String>,
    pub account: Option<String>,
    pub asset_type: Option<String>,
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub amount: Option<String>,
    pub into: Option<String>,
    pub source_amount: Option<String>,
    pub source_asset_code: Option<String>,
    pub source_asset_issuer: Option<String>,
    pub source_asset_type: Option<String>,
    pub path: Option<Vec<Path>>,
    pub destination_min: Option<String>,
    pub offer_id: Option<String>,
    pub buying_asset_type: Option<String>,
    pub buying_asset_code: Option<String>,
    pub buying_asset_issuer: Option<String>,
    pub price: Option<String>,
    pub price_r: Option<PriceRShortHand<u32>>,
    pub selling_asset_type: Option<String>,
    pub selling_asset_code: Option<String>,
    pub selling_asset_issuer: Option<String>,
    pub signer_key: Option<String>,
    pub signer_weight: Option<u32>,
    pub master_key_weight: Option<u32>,
    pub low_threshold: Option<u32>,
    pub med_threshold: Option<u32>,
    pub high_threshold: Option<u32>,
    pub home_domain: Option<String>,
    pub set_flags: Option<Vec<u32>>,
    pub set_flags_s: Option<Vec<String>>,
    pub clear_flags: Option<Vec<u32>>,
    pub clear_flags_s: Option<Vec<String>>,
    pub liquidity_pool_id: Option<String>,
    pub trustee: Option<String>,
    pub trustor: Option<String>,
    pub limit: Option<String>,
    pub authorize: Option<bool>,
    pub authorize_to_maintain_liabilities: Option<bool>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub bump_to: Option<String>,
    pub asset: Option<String>,
    pub claimants: Option<Claimant>,
    pub balance_id: Option<String>,
    pub claimant: Option<String>,
    pub sponsor: Option<String>,
    pub sponsored_id: Option<String>,
    pub begin_sponsor: Option<String>,
    pub reserves_max: Option<Vec<Reserve>>,
    pub reserves_min: Option<Vec<Reserve>>,
    pub min_price: Option<String>,
    pub min_price_r: Option<PriceRShortHand<u32>>,
    pub max_price: Option<String>,
    pub max_price_r: Option<PriceRShortHand<u32>>,
    pub reserves_deposited: Option<Vec<Reserve>>,
    pub shares_received: Option<String>,
    pub shares: Option<String>,
    pub reserves_received: Option<Vec<Reserve>>,
}
