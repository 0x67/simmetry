use diesel::{
    prelude::*,
    sql_types::{self, Jsonb, Text},
};
use serde::Serialize;
use serde_json::Value;

table! {
  f1_data (id) {
      id -> Text,
      data -> Jsonb,
  }
}

#[derive(Insertable)]
#[diesel(table_name = f1_data)]
pub struct F1Data<'a> {
    pub id: &'a str,
    pub data: &'a Value,
}
