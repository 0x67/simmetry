use bincode::{Decode, Encode};
#[cfg(feature = "db")]
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter};

#[derive(
    Hash,
    Default,
    Debug,
    Clone,
    Copy,
    Deserialize,
    Display,
    Serialize,
    Eq,
    PartialEq,
    Encode,
    Decode,
    EnumIter,
    AsRefStr,
)]
#[cfg_attr(feature = "db", derive(DbEnum))]
#[cfg_attr(feature = "db", DbValueStyle = "verbatim")]
pub enum F1Type {
    #[default]
    F12024,
    F12023,
    F12022,
}

#[cfg(feature = "db")]
pub(crate) mod export {
    pub use super::F1TypeMapping as F1Type;
}
