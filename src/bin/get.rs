use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
struct ProgramArgs {
    /// program hash
    hash: String,
}

#[tokio::main]
async fn main() {
    let args = ProgramArgs::parse();

    let data = Request { hash: args.hash };
    let result = reqwest::Client::new()
        .get("http://localhost:3000")
        .query(&data)
        .send()
        .await
        .unwrap();
    let response: Option<Response> = result.json().await.unwrap();
    println!("{:?}", response);
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Request {
    pub hash: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Response {
    pub hash: String,
    pub value: String,
}
