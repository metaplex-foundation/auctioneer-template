use anchor_lang::prelude::*;

pub const BID_SIZE: usize = 8 + 1 + 32;
pub const LISTING_CONFIG_SIZE: usize = 8 + 1 + 8 + 8 + BID_SIZE + 1 + 8 + 8 + 4 + 4 + 1;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub enum ListingConfigVersion {
    V0,
}

#[account]
pub struct ListingConfig {
    pub version: ListingConfigVersion,
}
