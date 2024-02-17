use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::io::{self, AsyncReadExt};

#[derive(Debug, Parser)]
struct ProgramArgs {
    /// program hash
    hash: String,

    /// value
    value: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = ProgramArgs::parse();

    let mut proof = String::new();
    io::stdin().read_to_string(&mut proof).await.unwrap();
    let data = Request {
        hash: args.hash,
        value: args.value,
        proof,
    };
    let result = reqwest::Client::new()
        .post("http://localhost:3000")
        .json(&data)
        .send()
        .await
        .unwrap();
    tracing::info!("{:?}", result);
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Request {
    pub hash: String,
    pub value: String,
    pub proof: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Response {
    pub hash: String,
    pub value: String,
}
