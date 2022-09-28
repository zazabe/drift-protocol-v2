use crate::error::{ClearingHouseResult, ErrorCode};
use crate::math::constants::{FEE_DENOMINATOR, FEE_PERCENTAGE_DENOMINATOR, QUOTE_PRECISION};
use crate::state::state::{FeeStructure, FeeTier};
use crate::validate;
use solana_program::msg;

pub fn validate_fee_structure(fee_structure: &FeeStructure) -> ClearingHouseResult {
    for (i, fee_tier) in fee_structure.fee_tiers.iter().enumerate() {
        validate_fee_tier(
            i,
            fee_tier,
            fee_structure.filler_reward_structure.reward_numerator,
        )?;
    }

    let is_filler_reward_valid = fee_structure.filler_reward_structure.reward_numerator <= 20
        && fee_structure.filler_reward_structure.reward_denominator == FEE_PERCENTAGE_DENOMINATOR; // <= 20%

    validate!(
        is_filler_reward_valid,
        ErrorCode::InvalidFeeStructure,
        "invalid filler reward numerator ({}) or denominator  ({})",
        fee_structure.filler_reward_structure.reward_numerator,
        fee_structure.filler_reward_structure.reward_denominator
    )?;

    validate!(
        fee_structure.flat_filler_fee <= QUOTE_PRECISION,
        ErrorCode::InvalidFeeStructure,
        "invalid flat filler fee {}",
        fee_structure.flat_filler_fee
    )?;

    Ok(())
}

pub fn validate_fee_tier(
    fee_tier_index: usize,
    fee_tier: &FeeTier,
    filler_reward_numerator: u32,
) -> ClearingHouseResult {
    let fee_valid = fee_tier.fee_numerator <= 100 && fee_tier.fee_denominator == FEE_DENOMINATOR; // <= 10bps

    validate!(
        fee_valid,
        ErrorCode::InvalidFeeStructure,
        "invalid fee numerator ({}) or denominator  ({})",
        fee_tier.fee_numerator,
        fee_tier.fee_denominator
    )?;

    let maker_rebate_valid = fee_tier.maker_rebate_numerator <= 30
        && fee_tier.maker_rebate_denominator == FEE_DENOMINATOR; // <= 3bps

    validate!(
        maker_rebate_valid,
        ErrorCode::InvalidFeeStructure,
        "invalid maker rebate numerator ({}) or denominator  ({})",
        fee_tier.maker_rebate_numerator,
        fee_tier.maker_rebate_denominator
    )?;

    let referee_discount_valid = fee_tier.referee_fee_numerator <= 20
        && fee_tier.referee_fee_denominator == FEE_PERCENTAGE_DENOMINATOR; // <= 20%

    validate!(
        referee_discount_valid,
        ErrorCode::InvalidFeeStructure,
        "invalid referee discount numerator ({}) or denominator  ({})",
        fee_tier.referee_fee_numerator,
        fee_tier.referee_fee_denominator
    )?;

    let referrer_reward_valid = fee_tier.referrer_reward_numerator <= 20
        && fee_tier.referrer_reward_denominator == FEE_PERCENTAGE_DENOMINATOR; // <= 20%

    validate!(
        referrer_reward_valid,
        ErrorCode::InvalidFeeStructure,
        "invalid referrer reward numerator ({}) or denominator  ({})",
        fee_tier.referrer_reward_numerator,
        fee_tier.referrer_reward_denominator
    )?;

    let taker_fee = fee_tier.fee_numerator * (100 - fee_tier.referee_fee_numerator) / 100;
    let fee_to_market = taker_fee
        - fee_tier.maker_rebate_numerator
        - taker_fee * (fee_tier.referrer_reward_numerator + filler_reward_numerator) / 100;

    validate!(
        fee_to_market <= fee_tier.fee_numerator,
        ErrorCode::InvalidFeeStructure,
        "invalid fee to market ({}) for index ({})",
        fee_tier.referrer_reward_numerator,
        fee_tier_index,
    )?;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::state::state::FeeStructure;
    use crate::validation::fee_structure::validate_fee_structure;

    #[test]
    fn default_fee_structures() {
        validate_fee_structure(&FeeStructure::perps_default()).unwrap();

        validate_fee_structure(&FeeStructure::spot_default()).unwrap();
    }
}