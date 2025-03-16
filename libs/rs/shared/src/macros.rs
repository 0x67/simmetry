#[cfg(feature = "db")]
#[macro_export]
macro_rules! diesel_enum_str {
    ($type:ty { $($variant:ident),* }) => {
        impl TryFrom<&str> for $type {
            type Error = String;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    $(stringify!($variant) => Ok(Self::$variant),)*
                    _ => Err(format!("Unknown {} variant: {}", stringify!($type), value)),
                }
            }
        }

        impl FromSql<Text, Pg> for $type {
            fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
                let t = <String as FromSql<Text, Pg>>::from_sql(bytes)?;
                Self::try_from(t.as_str()).map_err(|e| e.into())
            }
        }

        impl ToSql<Text, Pg> for $type {
            fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
                <str as ToSql<Text, Pg>>::to_sql(&self.to_string(), &mut out.reborrow())?;
                Ok(diesel::serialize::IsNull::No)
            }
        }
    };
}

#[cfg(feature = "db")]
#[macro_export]
macro_rules! diesel_enum_i32 {
    ($type:ty) => {
        impl ToSql<Integer, Pg> for $type {
            fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
                <i32 as ToSql<Integer, Pg>>::to_sql(&(*self as i32), &mut out.reborrow())?;
                Ok(IsNull::No)
            }
        }

        impl FromSql<Integer, Pg> for $type {
            fn from_sql(sql_value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
                let value = i32::from_sql(sql_value)?;
                if let Some(converted) = <$type>::from_repr(value as usize) {
                    Ok(converted)
                } else {
                    Err(format!("Unrecognized variant {} for {}", value, stringify!($type)).into())
                }
            }
        }
    };
}
