// admin instructions
pub mod approve_tree_owner;
pub mod init;

// test instructions

// public instructions
pub mod purchase_carbon_credits;
pub mod register_institution;
pub mod register_tree_owner;

// bring everything in scope
pub use {
    approve_tree_owner::*, init::*, purchase_carbon_credits::*, register_institution::*,
    register_tree_owner::*,
};
