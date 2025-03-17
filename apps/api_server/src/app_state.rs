use std::sync::Arc;

use deadpool_diesel::postgres::Pool;

use crate::services::forza::ForzaService;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub forza_service: Arc<ForzaService>,
}
