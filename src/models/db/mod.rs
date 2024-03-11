use serde_derive::{Deserialize, Serialize};

pub mod profile;
pub mod account;
pub mod note;
pub mod model;
pub mod printer;
pub mod file;

#[derive(diesel_derive_enum::DbEnum, Debug, Deserialize, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::ModifiedScale"]
pub enum ModifiedScale {
    NoMods,
    LightMods,
    MediumMods,
    HardMods,
    NewPrinter
}

#[derive(diesel_derive_enum::DbEnum, Debug, Deserialize, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::EventAudience"]
pub enum EventAudience {
    Public,
    Followers,
    Mentioned,
    Nobody
}