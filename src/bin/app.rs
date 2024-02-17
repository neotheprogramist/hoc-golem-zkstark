use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};

#[derive(Debug, Clone)]
struct AppState {
    pub db: Surreal<Db>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    let state = AppState { db };

    let app = Router::new()
        .route("/", get(get_handler))
        .route("/", post(post_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {:?}", listener);

    axum::serve(listener, app)
        .with_graceful_shutdown(hoc_golem_zkstark::shutdown_signal())
        .await
        .unwrap();
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Entry {
    hash: String,
    value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetQuery {
    pub hash: String,
}

async fn get_handler(
    State(state): State<AppState>,
    Query(query): Query<GetQuery>,
) -> Result<Json<Option<Entry>>, StatusCode> {
    let entry: Option<Entry> = state
        .db
        .select(("entry", query.hash.to_owned()))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(entry))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Data {
    hash: String,
    value: String,
    proof: String,
}

async fn post_handler(
    State(state): State<AppState>,
    Json(payload): Json<Data>,
) -> Result<Json<Option<Entry>>, StatusCode> {
    let proof = prefix_hex::decode(payload.proof).map_err(|_| StatusCode::BAD_REQUEST)?;
    tracing::info!("proof verifying...");
    if false == hoc_golem_zkstark::verify(proof) {
        return Err(StatusCode::FORBIDDEN);
    }

    let entry: Option<Entry> = if let Some(_) = state
        .db
        .select::<Option<Entry>>(("entry", payload.hash.to_owned()))
        .await
        .unwrap()
    {
        state
            .db
            .update(("entry", payload.hash.to_owned()))
            .content(Entry {
                hash: payload.hash.to_owned(),
                value: payload.value,
            })
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        state
            .db
            .create(("entry", payload.hash.to_owned()))
            .content(Entry {
                hash: payload.hash.to_owned(),
                value: payload.value,
            })
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    Ok(Json(entry))
}
