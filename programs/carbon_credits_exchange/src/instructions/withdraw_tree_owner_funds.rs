//! WithdrawTreeOwnerFunds instruction handler

use {
    crate::{
        error::HealthcareStaffingError,
        state::application::CarbonCreditsApplication,
        state::{configs::CarbonCreditsConfigs, deposit_base::DepositBase, tree_owner::TreeOwner},
    },
    anchor_lang::{prelude::*, system_program},
};

#[derive(Accounts)]
#[instruction(params: WithdrawTreeOwnerFundsParams)]
pub struct WithdrawTreeOwnerFunds<'info> {
    #[account(mut, has_one = owner, constraint = tree_owner.approval_status @ HealthcareStaffingError::InvalidTreeOwnerApprovalStatus)]
    pub tree_owner: Account<'info, TreeOwner>,
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
pub struct WithdrawTreeOwnerFundsParams {
    pub withdrawal_amount: u64, // withdrawal amount
}

pub fn withdraw_funds(
    ctx: Context<WithdrawTreeOwnerFunds>,
    params: &WithdrawTreeOwnerFundsParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.withdrawal_amount == 0 {
        return Err(HealthcareStaffingError::InvalidWithdrawalAmount.into());
    }

    let sys_program = &ctx.accounts.system_program;
    let deposit_account = &ctx.accounts.admin_deposit_account;
    let pda_auth = &mut ctx.accounts.admin_pda_auth;
    let sol_vault = &mut ctx.accounts.admin_sol_vault;

    let tree_owner = &mut ctx.accounts.tree_owner;
    let carbon_credits_configs = &mut ctx.accounts.carbon_credits_configs;

    let unit_cost_of_carbon_credit: u32 = carbon_credits_configs.unit_cost_of_carbon_credit;
    //let total_carbon_credits_configs: u32 = carbon_credits_configs.total_carbon_credits;
    let treasury_available_funds: u32 = carbon_credits_configs.available_funds;
    let available_funds: u32 = tree_owner.available_funds;

    // convert withdrawal_amount lamports to Sol
    let lamports = 1_000_000_000; // 1 SOL = 1,000,000,000 lamports
    let withdrawal_amount_sol = params
        .withdrawal_amount
        .checked_div(lamports)
        .ok_or(HealthcareStaffingError::InvalidArithmeticOperation)?;

    // Treasury's available funds should exceed withdrawal amount
    if treasury_available_funds as u64 > withdrawal_amount_sol {
    } else {
        return Err(HealthcareStaffingError::InsufficientTreasuryFunds.into());
    }

    // Tree owner's available funds should exceed withdrawal amount
    if available_funds as u64 > withdrawal_amount_sol {
    } else {
        return Err(HealthcareStaffingError::InsufficientFunds.into());
    }

    // Deduct withdrawn amount from treasury's available funds
    carbon_credits_configs.available_funds = treasury_available_funds
        .checked_sub(withdrawal_amount_sol as u32)
        .ok_or(HealthcareStaffingError::InvalidArithmeticOperation)?;

    // Deduct withdrawn amount from tree owner's available funds
    tree_owner.available_funds = available_funds
        .checked_sub(withdrawal_amount_sol as u32)
        .ok_or(HealthcareStaffingError::InvalidArithmeticOperation)?;

    let amount = params.withdrawal_amount;

    // Tree owner withdraws funds(equal to their carbon credits) from treasury vault
    let cpi_accounts = system_program::Transfer {
        from: sol_vault.to_account_info(),
        to: ctx.accounts.owner.to_account_info(),
    };

    let seeds = &[
        b"admin-sol-vault",
        pda_auth.to_account_info().key.as_ref(),
        &[deposit_account.admin_sol_vault_bump.unwrap()],
    ];

    let signer = &[&seeds[..]];

    let cpi = CpiContext::new_with_signer(sys_program.to_account_info(), cpi_accounts, signer);

    system_program::transfer(cpi, amount)?;

    Ok(())
}
