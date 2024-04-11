use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct CarbonCreditsConfigs {
    pub total_no_of_trees_planted: u32, // total no of trees planted
    pub total_carbon_credits: u32,      // total carbon credits available
    pub single_tree_to_carbon_credits_mapping: u8, // used to compute no of carbon credits from a single tree
    pub unit_cost_of_carbon_credit: u32,           // unit cost of carbon credit
    pub available_funds: u64, // available funds computed from the carbon credits purchased by companies
    pub active: bool,         // status of configs
}
