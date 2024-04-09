//! Init instruction handler

use {
    crate::{
        //error::PerpetualsError,
        //math,
        state::application::CarbonCreditsApplication,
        state::configs::CarbonCreditsConfigs,
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
        space = 8 + CarbonCreditsApplication::INIT_SPACE,
        seeds = [b"carbon-credits-configs"],
        bump
    )]
    pub carbon_credits_configs: Account<'info, CarbonCreditsConfigs>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitParams {
    pub single_tree_to_carbon_credits_mapping: u8, // used to compute no of carbon credits from a single tree
    pub unit_cost_of_carbon_credit: u32,           // unit cost of carbon credit
}

pub fn init(ctx: Context<Init>, params: &InitParams) -> Result<()> {
    let application = &mut ctx.accounts.application;
    let carbon_credits_configs = &mut ctx.accounts.carbon_credits_configs;
    // * - means dereferencing
    application.active = true;
    carbon_credits_configs.total_no_of_trees_planted = 0;
    carbon_credits_configs.total_carbon_credits = 0;
    carbon_credits_configs.single_tree_to_carbon_credits_mapping =
        params.single_tree_to_carbon_credits_mapping;
    carbon_credits_configs.unit_cost_of_carbon_credit = params.unit_cost_of_carbon_credit;
    carbon_credits_configs.active = true;

    Ok(())
}
