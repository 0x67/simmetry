use std::sync::Arc;

use crossbeam_channel::Sender;
use deadpool_diesel::postgres::Pool;
use rs_shared::database::models::forza::ForzaData;

use crate::services::forza::ForzaService;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub forza_service: Arc<ForzaService>,
    pub forza_data_sender: Sender<ForzaData>,
}
