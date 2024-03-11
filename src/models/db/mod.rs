use serde_derive::{Deserialize, Serialize};

pub mod account;
pub mod file;
pub mod model;
pub mod note;
pub mod printer;
pub mod profile;

#[derive(diesel_derive_enum::DbEnum, Debug, Deserialize, Serialize, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::ModifiedScale"]
pub enum ModifiedScale {
    NoMods,
    LightMods,
    MediumMods,
    HardMods,
    NewPrinter,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Deserialize, Serialize, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::EventAudience"]
pub enum EventAudience {
    Public,
    Followers,
    Mentioned,
    Nobody,
}
