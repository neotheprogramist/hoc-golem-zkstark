use tokio::io::{stdin, AsyncReadExt};

use hoc_golem_zkstark::verify;

#[tokio::main]
async fn main() {
    let mut proof = String::new();
    stdin().read_to_string(&mut proof).await.unwrap();

    let result = verify(prefix_hex::decode(proof).unwrap());
    println!("proof verified: {result}");
}
