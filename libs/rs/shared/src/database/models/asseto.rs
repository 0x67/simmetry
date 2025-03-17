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
pub enum AssettoCorsaType {
    #[default]
    AC1,
    ACC,
    ACEVO,
}

#[cfg(feature = "db")]
pub(crate) mod export {
    pub use super::AssettoCorsaTypeMapping as AssettoCorsaType;
}
