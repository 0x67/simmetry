use binrw::BinRead;

#[cfg(feature = "db")]
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    pg::{Pg, PgValue},
    serialize::*,
    sql_types::Integer,
};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, FromRepr};

#[non_exhaustive]
#[cfg_attr(feature = "db", derive(FromRepr, FromSqlRow))]
#[cfg_attr(feature = "db", diesel(sql_type = Integer))]
#[derive(BinRead, PartialEq, Display, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
#[br(little, repr(i32))]
pub enum ForzaCarClass {
    D,
    C,
    B,
    A,
    S1,
    S2,
    S3,
    X,
}

#[cfg(feature = "db")]
diesel_enum_i32!(ForzaCarClass);

#[non_exhaustive]
#[cfg_attr(feature = "db", derive(FromRepr, FromSqlRow))]
#[cfg_attr(feature = "db", diesel(sql_type = Integer))]
#[derive(
    BinRead, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug, Hash, Serialize, Deserialize,
)]
#[br(little, repr(i32))]
pub enum ForzaDriveType {
    FWD = 0,
    RWD = 1,
    AWD = 2,
}

#[cfg(feature = "db")]
diesel_enum_i32!(ForzaDriveType);
