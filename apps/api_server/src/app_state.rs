use std::sync::Arc;

use crate::services::forza::ForzaService;
use deadpool_diesel::postgres::Pool;
use rs_shared::database::models::forza::ForzaData;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone)]
pub struct AppState {
    pub redis_client: redis::Client,
    pub db_pool: Pool,
    pub forza_service: Arc<ForzaService>,
    pub forza_data_sender: UnboundedSender<ForzaData>,
}
