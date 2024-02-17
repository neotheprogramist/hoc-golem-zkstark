use serde::{Deserialize, Serialize};
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut proof = String::new();
    io::stdin().read_to_string(&mut proof).await.unwrap();

    let data = Request {
        hash: "1414999590036870372770778692906046516351208194453833185145989807125183271772"
            .to_string(),
        value: "1".to_string(),
        proof,
    };
    let result = reqwest::Client::new()
        .post("http://localhost:3000")
        .json(&data)
        .send()
        .await
        .unwrap();
    println!("{:?}", result);
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
