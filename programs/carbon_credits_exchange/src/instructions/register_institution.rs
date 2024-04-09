//! AddApplicant instruction handler

use {
    crate::{error::HealthcareStaffingError, state::institution::Institution},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: RegisterInstitutionParams)]
pub struct RegisterInstitution<'info> {
    // init means to create institution account
    // bump to use unique address for institution account
    #[account(
        init,
        payer = owner,
        space = 8 + Institution::INIT_SPACE,
        seeds = [b"institution", owner.key().as_ref()],
        bump
    )]
    pub institution: Account<'info, Institution>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterInstitutionParams {
    institution_type: u8,     //institution type
    institution_name: String, // institution name
    country: String,          // home country of institution
                              //active: bool,                      // status of institution
}

// institution name length
const INSTITUTION_NAME_LENGTH: usize = 30;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;

pub fn register_institution(
    ctx: Context<RegisterInstitution>,
    params: &RegisterInstitutionParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");

    /* EducationalInstitution = 1,
    NursingRegulatoryLicensingBody = 2,
    Commission = 3,
    HealthcareStaffingCompany = 4, */
    let is_valid_institution_type = if params.institution_type == 1
        || params.institution_type == 2
        || params.institution_type == 3
        || params.institution_type == 4
    {
        true
    } else {
        false
    };

    if !is_valid_institution_type {
        return Err(HealthcareStaffingError::InvalidInstitutionType.into());
    }
    if params.institution_name.as_bytes().len() > INSTITUTION_NAME_LENGTH {
        return Err(HealthcareStaffingError::ExceededInstitutionNameMaxLength.into());
    } else if params.institution_name.as_bytes().len() == 0 {
        return Err(HealthcareStaffingError::InvalidLength.into());
    }

    if params.country.as_bytes().len() != COUNTRY_LENGTH
        && params.country.as_bytes().len() != COUNTRY_LENGTH_2
    {
        return Err(HealthcareStaffingError::InvalidCountryLength.into());
    } else if params.country.as_bytes().len() == 0 {
        return Err(HealthcareStaffingError::InvalidLength.into());
    }

    let institution = &mut ctx.accounts.institution;
    // * - means dereferencing
    institution.owner = *ctx.accounts.owner.key;
    institution.institution_type = params.institution_type;
    institution.institution_name = params.institution_name.to_string();
    institution.country = params.country.to_string();
    institution.active = true;

    Ok(())
}
