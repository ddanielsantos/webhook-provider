pub mod subscription {
    use axum::{extract::State, response::IntoResponse, routing::put, Router};

    use crate::state::AppState;

    #[derive(serde::Deserialize)]
    struct SubscribeBody {
        subscribeTo: Vec<String>,
        secret: String,
        url: String,
    }

    async fn subscribe_handler(
        State(state): State<AppState>,
        axum::Json(body): axum::Json<SubscribeBody>,
    ) -> impl IntoResponse {
    }

    pub fn subscription_routes() -> Router<AppState> {
        Router::new().route("/subscribe", put(subscribe_handler))
    }
}
