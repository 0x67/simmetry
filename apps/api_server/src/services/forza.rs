use deadpool_diesel::postgres::Pool;
use diesel::RunQueryDsl;
use rs_shared::database::models::forza::ForzaTelemetry;

#[derive(Clone)]
pub struct ForzaService {
    pub pool: Pool,
}

impl ForzaService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn _create_forza_telemetry(
        &self,
        data: ForzaTelemetry,
    ) -> Result<ForzaTelemetry, diesel::result::Error> {
        use rs_shared::database::schema::forza_telemetry;

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
                diesel::insert_into(forza_telemetry::table)
                    .values(data)
                    .get_result::<ForzaTelemetry>(conn)
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

    pub async fn create_forza_telemetry_batch(
        &self,
        data: Vec<ForzaTelemetry>,
    ) -> Result<(), diesel::result::Error> {
        use rs_shared::database::schema::forza_telemetry;

        let pool = self.pool.clone();

        let conn = pool.get().await.map_err(|e| {
            eprintln!("Failed to get database connection: {:?}", e);
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        let _ = conn
            .interact(move |conn| {
                diesel::insert_into(forza_telemetry::table)
                    .values(data)
                    .execute(conn)
            })
            .await
            .map_err(|e| {
                eprintln!("Interact error: {:?}", e);
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string()),
                )
            })?;

        Ok(())
    }
}
