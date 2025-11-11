use axum::{Json, extract::State, routing::get};
use jsonwebtoken::jwk::JwkSet;

pub fn router(jwk_set: JwkSet) -> axum::Router {
    axum::Router::new()
        .route("/hello", get(hello))
        .route("/jwk", get(jwk))
        .with_state(jwk_set)
}

#[derive(serde::Serialize)]
struct Hello {
    message: String,
}

async fn hello() -> Json<Hello> {
    let hello = Hello {
        message: "Hello, World!".to_string(),
    };

    axum::Json(hello)
}

async fn jwk(State(jwk_set): State<JwkSet>) -> Json<JwkSet> {
    axum::Json(jwk_set)
}
