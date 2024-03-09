pub mod subscription {
    use axum::{response::IntoResponse, routing::put, Router};

    use crate::state::AppState;

    pub async fn subscribe_handler() -> impl IntoResponse {}

    pub fn subscription_routes() -> Router<AppState> {
        Router::new().route("/subscribe", put(subscribe_handler))
    }
}
