use std::convert::TryInto;

use crate::*;
use manager::Asset;

pub fn check_feed_update(
    assets: &Vec<Asset>,
    indexA: usize,
    indexB: usize,
    max_delay: u32,
    slot: u64,
) -> Result<()> {
    // Check assetA
    if (assets[indexA].last_update as u64) < slot - max_delay as u64 {
        return Err(ErrorCode::OutdatedOracle.into());
    }
    // Check assetB
    if (assets[indexB].last_update as u64) < slot - max_delay as u64 {
        return Err(ErrorCode::OutdatedOracle.into());
    }
    return Ok(());
}

pub fn div_up(a: u128, b: u128) -> u128 {
    (a + (b - 1)) / b
}