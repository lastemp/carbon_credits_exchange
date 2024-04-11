//! carbon_credits_exchange program entrypoint

pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("BBR9aQywAfCLEPNhq3djmyYPGLrxyUmFoYNpd14X4NM9");

#[program]
pub mod carbon_credits_exchange {
    use super::*;

    // admin instructions
    pub fn init(ctx: Context<Init>, params: InitParams) -> Result<()> {
        instructions::init(ctx, &params)
    }

    pub fn approve_tree_owner(
        ctx: Context<ApproveTreeOwner>,
        params: ApproveTreeOwnerParams,
    ) -> Result<()> {
        instructions::approve_tree_owner(ctx, &params)
    }

    // public instructions
    pub fn register_tree_owner(
        ctx: Context<RegisterTreeOwner>,
        params: RegisterTreeOwnerParams,
    ) -> Result<()> {
        instructions::register_tree_owner(ctx, &params)
    }

    pub fn register_institution(
        ctx: Context<RegisterInstitution>,
        params: RegisterInstitutionParams,
    ) -> Result<()> {
        instructions::register_institution(ctx, &params)
    }

    pub fn purchase_carbon_credits(
        ctx: Context<PurchaseCarbonCredits>,
        params: PurchaseCarbonCreditsParams,
    ) -> Result<()> {
        instructions::purchase_carbon_credits(ctx, &params)
    }

    pub fn withdraw_tree_owner_funds(
        ctx: Context<WithdrawTreeOwnerFunds>,
        params: WithdrawTreeOwnerFundsParams,
    ) -> Result<()> {
        instructions::withdraw_funds(ctx, &params)
    }
}
