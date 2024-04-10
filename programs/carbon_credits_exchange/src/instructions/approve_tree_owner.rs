//approve_tree_owner.
//! AddApplicant instruction handler

use {
    crate::{
        error::HealthcareStaffingError, state::configs::CarbonCreditsConfigs,
        state::tree_owner::TreeOwner,
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: ApproveTreeOwnerParams)]
pub struct ApproveTreeOwner<'info> {
    // mut makes it changeble (mutable)
    /// CHECK: tree_owner account for active status
    #[account(
        mut, constraint = tree_owner.active @ HealthcareStaffingError::InvalidTreeOwnerStatus
    )]
    pub tree_owner: Account<'info, TreeOwner>,
    /// CHECK: carbon_credits_configs account for active status
    #[account(
        mut, constraint = carbon_credits_configs.active @ HealthcareStaffingError::InvalidCarbonCreditsConfigsStatus
    )]
    pub carbon_credits_configs: Account<'info, CarbonCreditsConfigs>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ApproveTreeOwnerParams {
    no_of_trees: u32,      // no of trees planted
    approval_status: bool, // approval status of tree owner
}

pub fn approve_tree_owner(
    ctx: Context<ApproveTreeOwner>,
    params: &ApproveTreeOwnerParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.no_of_trees == 0 {
        return Err(HealthcareStaffingError::InvalidNoofTrees.into());
    }

    let tree_owner = &mut ctx.accounts.tree_owner;
    let carbon_credits_configs = &mut ctx.accounts.carbon_credits_configs;
    let single_tree_to_carbon_credits_mapping =
        carbon_credits_configs.single_tree_to_carbon_credits_mapping as u32;
    let total_no_of_trees_planted = carbon_credits_configs.total_no_of_trees_planted;
    let total_carbon_credits = carbon_credits_configs.total_carbon_credits;
    let no_of_trees = params.no_of_trees;
    let computed_carbon_credits = no_of_trees
        .checked_mul(single_tree_to_carbon_credits_mapping)
        .ok_or(HealthcareStaffingError::InvalidArithmeticOperation)?;

    tree_owner.approval_status = params.approval_status;
    tree_owner.no_of_trees = no_of_trees;

    carbon_credits_configs.total_no_of_trees_planted = total_no_of_trees_planted
        .checked_add(no_of_trees)
        .ok_or(HealthcareStaffingError::InvalidArithmeticOperation)?;

    carbon_credits_configs.total_carbon_credits = total_carbon_credits
        .checked_add(computed_carbon_credits)
        .ok_or(HealthcareStaffingError::InvalidArithmeticOperation)?;
    Ok(())
}
