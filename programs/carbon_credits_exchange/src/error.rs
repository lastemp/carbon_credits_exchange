//! Error types

use anchor_lang::prelude::*;

#[error_code]
pub enum HealthcareStaffingError {
    // applicant
    #[msg("Exceeded full names max length")]
    ExceededFullNamesMaxLength,
    #[msg("Exceeded date of birth max length")]
    ExceededDateOfBirthMaxLength,
    #[msg("Exceeded hospital max length")]
    ExceededHospitalMaxLength,
    #[msg("Country must have length of two or three")]
    InvalidCountryLength,
    #[msg("Exceeded transcript max length")]
    ExceededTranscriptMaxLength,
    #[msg("Exceeded certificate max length")]
    ExceededCertificateMaxLength,
    #[msg("Exceeded license max length")]
    ExceededLicenseMaxLength,
    #[msg("National id no must have a value greater than zero.")]
    InvalidNationalIdNo,
    #[msg("license no must have a value greater than zero.")]
    InvalidLicenseNo,

    // institution
    #[msg("Institution type must have either of these values 1,2,3 or 4.")]
    InvalidInstitutionType,
    #[msg("Institution type does not match the specifed institution.")]
    MismatchedInstitutionType,
    #[msg("Exceeded institution name max length")]
    ExceededInstitutionNameMaxLength,

    // application
    #[msg("Application must be submitted by applicant in previous step to proceed.")]
    InvalidApplicationSubmissionStatus,

    // approve applicant
    #[msg("Exceeded Declined reason max length")]
    ExceededDeclinedReasonMaxLength,
    #[msg("Declined reason must have value if approval is declined")]
    InvalidDeclinedReason,
    #[msg("Approval must be done in previous step to proceed.")]
    InvalidApprovalStatus,

    // invalid length
    #[msg("Item must have a length greater than zero.")]
    InvalidLength,
}
