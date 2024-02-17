use platinum_prover::air::verify_cairo_proof;
use stark_platinum_prover::proof::options::{ProofOptions, SecurityLevel};
use tokio::io::{stdin, AsyncReadExt};


// taken from https://github.com/lambdaclass/lambdaworks/blob/main/provers/cairo/src/main.rs
#[tokio::main]
async fn main() {
    let mut proof: Vec<_> = Vec::new();
    stdin().read_to_end(&mut proof).await.unwrap();
    let proof_options = ProofOptions::new_secure(SecurityLevel::Conjecturable100Bits, 3);

    let mut bytes = proof.as_slice();
    if bytes.len() < 8 {
        eprintln!("Error reading proof");
        return;
    }

    // Proof len was stored as an u32, 4u8 needs to be read
    let proof_len = u32::from_le_bytes(bytes[0..4].try_into().unwrap()) as usize;

    bytes = &bytes[4..];
    if bytes.len() < proof_len {
        eprintln!("Error reading proof");
        return;
    }

    let Ok((proof, _)) =
        bincode::serde::decode_from_slice(&bytes[0..proof_len], bincode::config::standard())
    else {
        println!("Error reading proof");
        return;
    };
    bytes = &bytes[proof_len..];

    let Ok((pub_inputs, _)) = bincode::serde::decode_from_slice(bytes, bincode::config::standard())
    else {
        println!("Error reading proof");
        return;
    };

    let result = verify_cairo_proof(&proof, &pub_inputs, &proof_options);
    println!("proof verified: {result}");
}
