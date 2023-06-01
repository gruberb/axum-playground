use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Json;
use axum::Router;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
struct ValidatedNodes {
    nodes: Vec<PeerId>,
}

#[derive(Clone, Deserialize, Serialize)]
struct PeerId(String);

#[tokio::main]
pub async fn main() {
    let data = std::fs::read_to_string("validated_nodes.json").expect("Unable to read file");

    let nodes: ValidatedNodes =
        serde_json::from_str(&data).expect("JSON does not have correct format.");

    let app = Router::new().route("/", get(root));
    let app_with_state = app.with_state(nodes);

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app_with_state.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}

async fn root(State(state): State<ValidatedNodes>) -> impl IntoResponse {
    let state = state.clone();
    Json(state)
}
