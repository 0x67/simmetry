use deadpool_diesel::postgres::Pool;
use diesel::RunQueryDsl;
use rs_shared::database::models::forza::ForzaData;

#[derive(Clone)]
pub struct ForzaService {
    pub pool: Pool,
}

impl ForzaService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn create_forza_data(
        &self,
        data: ForzaData,
    ) -> Result<ForzaData, diesel::result::Error> {
        use rs_shared::database::schema::forza_data;

        let pool = self.pool.clone();

        let conn = pool.get().await.map_err(|e| {
            eprintln!("Failed to get database connection: {:?}", e);
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;
        let result = conn
            .interact(move |conn| {
                diesel::insert_into(forza_data::table)
                    .values(data)
                    .get_result::<ForzaData>(conn)
            })
            .await
            .map_err(|e| {
                eprintln!("Interact error: {:?}", e);
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string()),
                )
            })??;

        Ok(result)
    }
}
