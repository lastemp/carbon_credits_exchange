//! RegisterTreeOwner instruction handler

use {
    crate::{
        error::HealthcareStaffingError,
        state::tree_owner::{GpsCoordinates, TreeOwner},
    },
    anchor_lang::prelude::*,
    //anchor_spl::token::{Token, TokenAccount},
    //solana_program::program_error::ProgramError,
};

#[derive(Accounts)]
#[instruction(params: RegisterTreeOwnerParams)]
pub struct RegisterTreeOwner<'info> {
    // init means to create tree_owner account
    // bump to use unique address for tree_owner account
    #[account(
        init,
        payer = owner,
        space = 8 + TreeOwner::INIT_SPACE,
        seeds = [b"tree-owner", owner.key().as_ref()],
        bump
    )]
    pub tree_owner: Account<'info, TreeOwner>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterTreeOwnerParams {
    national_id_no: u32, // national id no
    full_names: String,  // full names i.e first name, middlename, surname
    //land_coordinates: GpsCoordinates, // land coordinates
    //no_of_trees: u32,                 // no of trees planted
    country: String, // home country of tree owner
}

// full names length
const FULL_NAMES_LENGTH: usize = 50;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;

pub fn register_tree_owner(
    ctx: Context<RegisterTreeOwner>,
    params: &RegisterTreeOwnerParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.national_id_no == 0 {
        return Err(HealthcareStaffingError::InvalidNationalIdNo.into());
    }
    if params.full_names.as_bytes().len() > FULL_NAMES_LENGTH {
        return Err(HealthcareStaffingError::ExceededFullNamesMaxLength.into());
    } else if params.full_names.as_bytes().len() == 0 {
        return Err(HealthcareStaffingError::InvalidLength.into());
    }

    if params.country.as_bytes().len() != COUNTRY_LENGTH
        && params.country.as_bytes().len() != COUNTRY_LENGTH_2
    {
        return Err(HealthcareStaffingError::InvalidCountryLength.into());
    } else if params.country.as_bytes().len() == 0 {
        return Err(HealthcareStaffingError::InvalidLength.into());
    }

    let tree_owner = &mut ctx.accounts.tree_owner;
    // * - means dereferencing
    tree_owner.owner = *ctx.accounts.owner.key;
    tree_owner.national_id_no = params.national_id_no;
    tree_owner.full_names = params.full_names.to_string();
    tree_owner.country = params.country.to_string();
    tree_owner.active = true;

    Ok(())
}
