use std::sync::Arc;

use deadpool_diesel::{postgres::Manager, Pool};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Manager>,
}
