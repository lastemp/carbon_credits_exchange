//! Init instruction handler

use {
    crate::{
        error::HealthcareStaffingError,
        state::{
            application::CarbonCreditsApplication, configs::CarbonCreditsConfigs,
            deposit_base::DepositBase,
        },
    },
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: InitParams)]
pub struct Init<'info> {
    // init means to create application account
    // bump to use unique address for application account
    #[account(
        init,
        payer = owner,
        space = 8 + CarbonCreditsApplication::INIT_SPACE,
        seeds = [b"application"],
        bump
    )]
    pub application: Account<'info, CarbonCreditsApplication>,
    #[account(
        init,
        payer = owner,
        space = 8 + CarbonCreditsConfigs::INIT_SPACE,
        seeds = [b"carbon-credits-configs"],
        bump
    )]
    pub carbon_credits_configs: Account<'info, CarbonCreditsConfigs>,
    #[account(init, payer = owner, space = 8 + DepositBase::INIT_SPACE,
        constraint = !admin_deposit_account.is_initialized @ HealthcareStaffingError::AccountAlreadyInitialized
    )]
    pub admin_deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"admin-auth", admin_deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub admin_pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"admin-sol-vault", admin_pda_auth.key().as_ref()], bump)]
    pub admin_sol_vault: SystemAccount<'info>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitParams {
    pub single_tree_to_carbon_credits_mapping: u8, // used to compute no of carbon credits from a single tree
    pub unit_cost_of_carbon_credit: u32,           // unit cost of carbon credit
    pub tree_owners_share_cost: f32, // tree owners share costthe carbon credits purchased by companies
}

pub fn init(ctx: Context<Init>, params: &InitParams) -> Result<()> {
    msg!("Validate inputs");
    if params.single_tree_to_carbon_credits_mapping == 0 {
        return Err(HealthcareStaffingError::InvalidSingleTreeToCarbonCreditsMapping.into());
    }
    if params.unit_cost_of_carbon_credit == 0 {
        return Err(HealthcareStaffingError::InvalidUnitCostOfCarbonCredit.into());
    }
    /* if params.tree_owners_share_cost == 0.0 {
        return Err(HealthcareStaffingError::InvalidTreeOwnersShareCost.into());
    } */

    let deposit_account = &mut ctx.accounts.admin_deposit_account;
    let application = &mut ctx.accounts.application;
    let carbon_credits_configs = &mut ctx.accounts.carbon_credits_configs;

    // admin deposit account
    deposit_account.owner = *ctx.accounts.owner.key;
    deposit_account.admin_auth_bump = ctx.bumps.admin_pda_auth;
    deposit_account.admin_sol_vault_bump = Some(ctx.bumps.admin_sol_vault);
    deposit_account.is_initialized = true;

    // application
    application.active = true;

    // carbon credits configs
    carbon_credits_configs.total_no_of_trees_planted = 0;
    carbon_credits_configs.total_carbon_credits = 0;
    carbon_credits_configs.single_tree_to_carbon_credits_mapping =
        params.single_tree_to_carbon_credits_mapping;
    carbon_credits_configs.unit_cost_of_carbon_credit = params.unit_cost_of_carbon_credit;
    //carbon_credits_configs.tree_owners_share_cost = params.tree_owners_share_cost;
    carbon_credits_configs.active = true;

    Ok(())
}
