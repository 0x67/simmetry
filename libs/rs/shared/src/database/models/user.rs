use chrono::NaiveDateTime;

#[cfg(feature = "db")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "db")]
use crate::database::schema::users;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    derive(Insertable, Selectable, Queryable, Identifiable)
)]
#[cfg_attr(feature = "db", diesel(table_name = users))]
#[cfg_attr(feature = "db", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
