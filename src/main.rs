use std::io::{stdin, Read};

use clap::Parser;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

mod verify;

#[derive(Parser)]
struct VerifierArgs {
    pub proof_path: String,
}

#[tokio::main]
async fn main() {
    let mut proof = Vec::new();
    stdin().read_to_end(&mut proof).unwrap();
    let db = Surreal::new::<Mem>(()).await.unwrap();


    // build our application with a route
    let app = Router::new().route("/", get(handler));
    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
