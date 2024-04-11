//! PurchaseCarbonCredits instruction handler

use {
    crate::{
        error::HealthcareStaffingError,
        state::application::CarbonCreditsApplication,
        state::{
            configs::CarbonCreditsConfigs, deposit_base::DepositBase, institution::Institution,
        },
    },
    anchor_lang::{prelude::*, system_program},
};

#[derive(Accounts)]
#[instruction(params: PurchaseCarbonCreditsParams)]
pub struct PurchaseCarbonCredits<'info> {
    #[account(mut, has_one = owner)]
    pub institution: Account<'info, Institution>,
    // mut makes it changeble (mutable)
    /// CHECK: carbon_credits_configs account for active status
    #[account(
        mut, constraint = carbon_credits_configs.active @ HealthcareStaffingError::InvalidCarbonCreditsConfigsStatus
    )]
    pub carbon_credits_configs: Account<'info, CarbonCreditsConfigs>,
    #[account(
        mut, constraint = carbon_credits_application.active @ HealthcareStaffingError::InvalidApplicationActiveStatus
    )]
    pub carbon_credits_application: Account<'info, CarbonCreditsApplication>,
    //admin accs
    #[account(mut,
        constraint = admin_deposit_account.is_initialized @ HealthcareStaffingError::AccountNotInitialized
    )]
    pub admin_deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"admin-auth", admin_deposit_account.key().as_ref()], bump = admin_deposit_account.admin_auth_bump)]
    /// CHECK: no need to check this.
    pub admin_pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"admin-sol-vault", admin_pda_auth.key().as_ref()], bump = admin_deposit_account.admin_sol_vault_bump.unwrap())]
    pub admin_sol_vault: SystemAccount<'info>,
    //admin accs
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PurchaseCarbonCreditsParams {
    pub carbon_credits: u32, // Size/Capacity of carbon credits to be purchased
}

pub fn purchase_carbon_credits(
    ctx: Context<PurchaseCarbonCredits>,
    params: &PurchaseCarbonCreditsParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.carbon_credits == 0 {
        return Err(HealthcareStaffingError::InvalidCarbonCredits.into());
    }

    // do a check to determine if carbon_credits are available in carbon_credits_configs

    let deposit_auth = &ctx.accounts.owner;
    let sys_program = &ctx.accounts.system_program;

    let carbon_credits_application = &mut ctx.accounts.carbon_credits_application;
    let carbon_credits_configs = &mut ctx.accounts.carbon_credits_configs;

    let unit_cost_of_carbon_credit: u32 = carbon_credits_configs.unit_cost_of_carbon_credit;
    let total_carbon_credits_configs: u32 = carbon_credits_configs.total_carbon_credits;
    let available_funds: u32 = carbon_credits_configs.available_funds;
    let total_carbon_credits: u32 = carbon_credits_application.total_carbon_credits;
    let carbon_credits = params.carbon_credits;

    carbon_credits_application.applicant = *ctx.accounts.owner.key;

    // Check that carbon credits is available
    if total_carbon_credits_configs == 0 {
        return Err(HealthcareStaffingError::InsufficientCarbonCredits.into());
    }

    carbon_credits_application.total_carbon_credits = total_carbon_credits
        .checked_add(carbon_credits)
        .ok_or(HealthcareStaffingError::InvalidArithmeticOperation)?;

    carbon_credits_configs.total_carbon_credits = total_carbon_credits_configs
        .checked_sub(carbon_credits)
        .ok_or(HealthcareStaffingError::InvalidArithmeticOperation)?;

    let total_purchase_amount = unit_cost_of_carbon_credit
        .checked_mul(carbon_credits)
        .ok_or(HealthcareStaffingError::InvalidArithmeticOperation)?;

    // Lets increment treasury available_funds with total_purchase_amount for carbon credits
    // the purchase is done by company
    carbon_credits_configs.available_funds = available_funds
        .checked_add(total_purchase_amount)
        .ok_or(HealthcareStaffingError::InvalidArithmeticOperation)?;

    carbon_credits_application.total_purchase_amount = total_purchase_amount;

    let lamports: u32 = 1_000_000_000; // 1 SOL = 1,000,000,000 lamports
    let _amount = total_purchase_amount
        .checked_mul(lamports)
        .ok_or(HealthcareStaffingError::InvalidArithmeticOperation)?;

    // transfer sol from carbon credits purchaser to treasury vault
    let cpi_accounts = system_program::Transfer {
        from: deposit_auth.to_account_info(),
        to: ctx.accounts.admin_sol_vault.to_account_info(),
    };

    let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

    system_program::transfer(cpi, _amount.into())?;

    Ok(())
}
