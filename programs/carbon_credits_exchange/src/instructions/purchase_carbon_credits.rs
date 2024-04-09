//! PurchaseCarbonCredits instruction handler

use {
    crate::{
        error::HealthcareStaffingError, state::application::CarbonCreditsApplication,
        state::configs::CarbonCreditsConfigs, state::institution::Institution,
        state::tree_owner::TreeOwner,
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: PurchaseCarbonCreditsParams)]
pub struct PurchaseCarbonCredits<'info> {
    #[account(mut, has_one = owner)]
    pub institution: Account<'info, Institution>,
    // mut makes it changeble (mutable)
    /// CHECK: carbon_credits_configs account for active status
    #[account(
        mut, constraint = carbon_credits_configs.active @ HealthcareStaffingError::InvalidApplicationSubmissionStatus
    )]
    pub carbon_credits_configs: Account<'info, CarbonCreditsConfigs>,
    #[account(
        mut, constraint = carbon_credits_application.active @ HealthcareStaffingError::InvalidApplicationSubmissionStatus
    )]
    pub carbon_credits_application: Account<'info, CarbonCreditsApplication>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PurchaseCarbonCreditsParams {
    pub carbon_credits: u32, // Size/Capacity carbon credits to be purchased
}

pub fn purchase_carbon_credits(
    ctx: Context<PurchaseCarbonCredits>,
    params: &PurchaseCarbonCreditsParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.carbon_credits == 0 {
        return Err(HealthcareStaffingError::InvalidNationalIdNo.into());
    }

    // do a check to determine if carbon_credits are available in carbon_credits_configs

    let carbon_credits_application = &mut ctx.accounts.carbon_credits_application;
    let carbon_credits_configs = &mut ctx.accounts.carbon_credits_configs;

    let unit_cost_of_carbon_credit: u32 = carbon_credits_configs.unit_cost_of_carbon_credit;
    let total_carbon_credits_configs: u32 = carbon_credits_configs.total_carbon_credits;
    let total_carbon_credits: u32 = carbon_credits_application.total_carbon_credits;
    let carbon_credits = params.carbon_credits;

    carbon_credits_application.applicant = *ctx.accounts.owner.key;

    carbon_credits_application.total_carbon_credits = total_carbon_credits
        .checked_add(carbon_credits)
        .ok_or(HealthcareStaffingError::InvalidNationalIdNo)?; // + carbon_credits;

    carbon_credits_configs.total_carbon_credits = total_carbon_credits_configs
        .checked_sub(carbon_credits)
        .ok_or(HealthcareStaffingError::ExceededLicenseMaxLength)?; // - carbon_credits;

    carbon_credits_application.total_purchase_amount = unit_cost_of_carbon_credit
        .checked_mul(carbon_credits)
        .ok_or(HealthcareStaffingError::InvalidCountryLength)?; // * carbon_credits;

    Ok(())
}
