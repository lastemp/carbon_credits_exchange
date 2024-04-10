use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct CarbonCreditsApplication {
    pub applicant: Pubkey,          // publickey of the applicant
    pub total_carbon_credits: u32,  // total carbon credits to be purchased by institution
    pub total_purchase_amount: u32, // total purchase amount for the carbon credits
    pub active: bool,               // status of application
                                    //pub submitted: bool,            // Indicates that the application was submitted by applicant
}
