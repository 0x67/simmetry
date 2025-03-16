use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use rs_shared::packets::forza::schema::{InsertF1Data, QueryF1Data};

pub fn create_f1_data(conn: &mut PgConnection, data: &InsertF1Data) {
    use rs_shared::packets::forza::schema::f1_data::dsl::f1_data;

    diesel::insert_into(f1_data)
        .values(data)
        .returning(QueryF1Data::as_returning())
        .get_result(conn)
        .expect("Error saving new post");
}
