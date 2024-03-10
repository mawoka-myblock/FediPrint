use crate::models::db::account::FullAccount;
use crate::models::db::profile::FullProfile;

pub mod profile;
pub mod account;
pub mod note;
pub mod model;


pub struct AccountWithProfile<'a> {
    profile: &'a FullProfile,
    account: &'a FullAccount
}

pub enum ModifiedScale {
    NoMods,
    LightMods,
    MediumMods,
    HardMods,
    NewPrinter
}

pub enum EventAudience {
    Public,
    Followers,
    Mentioned,
    Nobody
}