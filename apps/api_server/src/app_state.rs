use std::sync::Arc;

use crate::services::forza::ForzaService;
use deadpool_diesel::postgres::Pool;
use rs_shared::database::models::{f1::F1Telemetry, forza::ForzaTelemetry};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone)]
pub struct AppState {
    pub redis_client: redis::Client,
    pub db_pool: Pool,
    pub forza_service: Arc<ForzaService>,
    pub forza_telemetry_sender: UnboundedSender<ForzaTelemetry>,
    pub f1_telemetry_sender: UnboundedSender<F1Telemetry>,
    pub file_writer_sender: UnboundedSender<Vec<u8>>,
}
