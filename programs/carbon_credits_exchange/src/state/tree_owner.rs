use anchor_lang::prelude::*;

//#[account]
//#[derive(Default, Debug, InitSpace)]
#[derive(Default, Debug, AnchorSerialize, AnchorDeserialize, Copy, Clone, InitSpace)]
pub struct GpsCoordinates {
    pub latitude: f32,  // latitude
    pub longitude: f32, // longitude
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
}
