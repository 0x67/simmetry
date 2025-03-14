use diesel::{prelude::*, sql_types::Jsonb};
use serde::Serialize;

table! {
  f1_data (id) {
      id -> Text,
      data -> Jsonb,
  }
}

#[derive(Serialize, Selectable, Queryable)]
#[diesel(table_name = f1_data)]
pub struct F1Data {
    pub id: String,
    pub data: Jsonb,
}
