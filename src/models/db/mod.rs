use serde_derive::{Deserialize, Serialize};

pub mod account;
pub mod file;
pub mod model;
pub mod note;
pub mod printer;
pub mod profile;

#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "modified_scale", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModifiedScale {
    NoMods,
    LightMods,
    MediumMods,
    HardMods,
    NewPrinter,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "event_audience", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventAudience {
    Public,
    Followers,
    Mentioned,
    Nobody,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "model_license", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModelLicense {
    CcPd,
    CcAttr,
    CcAttrSa,
    CcAttrNd,
    CcAttrNc,
    CcAttrNcSa,
    CcAttrNcNd,
    Gpl2,
    Gpl3,
    GnuLesser,
    Bsd,
    Sdfl,
}
