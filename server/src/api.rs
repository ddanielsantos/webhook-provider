use axum::Router;

use crate::domains::subscription;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/api/subscribe", subscription::subscription_routes())
}
