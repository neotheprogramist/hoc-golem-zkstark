use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let data = Request {
        hash: "1414999590036870372770778692906046516351208194453833185145989807125183271772"
            .to_string(),
    };
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
