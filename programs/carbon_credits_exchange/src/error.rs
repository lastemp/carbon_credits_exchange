//! Error types

use anchor_lang::prelude::*;

#[error_code]
pub enum HealthcareStaffingError {
    // applicant
    #[msg("Exceeded full names max length")]
    ExceededFullNamesMaxLength,
    #[msg("Country must have length of two or three")]
    InvalidCountryLength,
    #[msg("National id no must have a value greater than zero.")]
    InvalidNationalIdNo,
    #[msg("Single tree to carbon credits mapping must have a value greater than zero.")]
    InvalidSingleTreeToCarbonCreditsMapping,
    #[msg("Unit cost of carbon credit must have a value greater than zero.")]
    InvalidUnitCostOfCarbonCredit,
    #[msg("Tree owners share cost must have a value greater than zero.")]
    InvalidTreeOwnersShareCost,
    #[msg("No of trees must have a value greater than zero.")]
    InvalidNoofTrees,
    #[msg("Carbon credits must have a value greater than zero.")]
    InvalidCarbonCredits,
    #[msg("Arithmetic operation failed.")]
    InvalidArithmeticOperation,
    #[msg("Tree owner has no active status.")]
    InvalidTreeOwnerStatus,
    #[msg("Tree owner has no approval status.")]
    InvalidTreeOwnerApprovalStatus,
    #[msg("Gps coordinates has invalid values.")]
    InvalidGpsCoordinates,

    // institution
    #[msg("Institution type must have either of these values 1,2,3 or 4.")]
    InvalidInstitutionType,
    #[msg("Institution type does not match the specifed institution.")]
    MismatchedInstitutionType,
    #[msg("Exceeded institution name max length")]
    ExceededInstitutionNameMaxLength,

    // carbon credits configs
    #[msg("Carbon credits configs has no active status.")]
    InvalidCarbonCreditsConfigsStatus,

    // carbon credits application
    #[msg("Carbon credits application has no active status.")]
    InvalidApplicationActiveStatus,

    // invalid length
    #[msg("Item must have a length greater than zero.")]
    InvalidLength,

    // amount
    #[msg("Invalid withdrawal amount.")]
    InvalidWithdrawalAmount,
    #[msg("Insufficient treasury funds.")]
    InsufficientTreasuryFunds,
    #[msg("Insufficient funds.")]
    InsufficientFunds,

    // carbon credits
    #[msg("Insufficient carbon credits.")]
    InsufficientCarbonCredits,

    // deposit base
    #[msg("Account is not initialized.")]
    AccountNotInitialized,
    #[msg("Account is already initialized.")]
    AccountAlreadyInitialized,
}
