use anchor_lang::prelude::*;

//#[account]
//#[derive(Default, Debug, InitSpace)]
#[derive(Default, Debug, AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct GpsCoordinates {
    #[max_len(10)]
    pub latitude: String, // latitude
    #[max_len(10)]
    pub longitude: String, // longitude
}

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct TreeOwner {
    pub owner: Pubkey,       // publickey of the tree owner
    pub national_id_no: u32, // national id no
    #[max_len(50)]
    pub full_names: String, // full names i.e first name, middlename, surname
    pub land_coordinates: GpsCoordinates, // land coordinates
    pub no_of_trees: u32,    // no of trees planted
    #[max_len(3)]
    pub country: String, // home country of tree owner
    pub active: bool,        // status of tree owner
    pub approval_status: bool, // Indicates approval status of tree owner
    pub computed_carbon_credits: u32, // computed carbon credits for the total no of trees planted
    pub available_funds: u32, // available funds computed from the carbon_credits issued to tree owner
}
