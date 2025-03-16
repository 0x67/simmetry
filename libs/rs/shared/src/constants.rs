use bincode::{Decode, Encode};

#[cfg(feature = "db")]
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{Output, ToSql},
    sql_types::Text,
};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

pub const F1_2024_UDP_PORT: u16 = 20001;
pub const F1_2023_UDP_PORT: u16 = 20002;
pub const F1_2022_UDP_PORT: u16 = 20003;

pub const FH5_UDP_PORT: u16 = 20011;
pub const FH4_UDP_PORT: u16 = 20012;

pub const AC_UDP_PORT: u16 = 20021;
pub const ACC_UDP_PORT: u16 = 20022;
pub const ACEVO_UDP_PORT: u16 = 20023;

#[cfg_attr(feature = "db", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "db", diesel(sql_type = Text))]
#[derive(
    Debug,
    Hash,
    Copy,
    Serialize,
    Display,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
    Encode,
    Decode,
    EnumIter,
)]
pub enum GameType {
    F12024,
    F12023,
    F12022,
    FH5,
    FH4,
    FM7,
    FM8,
    AC,
    ACC,
    ACEVO,
}

#[cfg(feature = "db")]
diesel_enum_str!(GameType {
    F12024,
    F12023,
    F12022,
    FH5,
    FH4,
    FM7,
    FM8,
    AC,
    ACC,
    ACEVO
});
