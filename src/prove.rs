use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use platinum_prover::air::{generate_cairo_proof, PublicInputs};
use platinum_prover::runner::run::generate_prover_args_from_trace;
use stark_platinum_prover::proof::{options::ProofOptions, stark::StarkProof};
use tokio::time::Instant;

pub fn generate_proof_from_trace(
    trace_bin_path: &str,
    memory_bin_path: &str,
    proof_options: &ProofOptions,
) -> Option<(
    StarkProof<Stark252PrimeField, Stark252PrimeField>,
    PublicInputs,
)> {
    // ## Generating the prover args
    let timer = Instant::now();
    let Ok((main_trace, pub_inputs)) =
        generate_prover_args_from_trace(trace_bin_path, memory_bin_path)
    else {
        eprintln!("Error generating prover args");
        return None;
    };
    println!("  Time spent: {:?} \n", timer.elapsed());

    // ## Prove
    let timer = Instant::now();
    println!("Making proof ...");
    let proof = match generate_cairo_proof(&main_trace, &pub_inputs, proof_options) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("Error generating proof: {:?}", err);
            return None;
        }
    };
    println!("  Time spent in proving: {:?} \n", timer.elapsed());

    Some((proof, pub_inputs))
}

pub fn write_proof(
    proof: StarkProof<Stark252PrimeField, Stark252PrimeField>,
    pub_inputs: PublicInputs,
    proof_path: String,
) {
    let mut bytes = vec![];
    let proof_bytes: Vec<u8> =
        bincode::serde::encode_to_vec(proof, bincode::config::standard()).unwrap();

    let pub_inputs_bytes: Vec<u8> =
        bincode::serde::encode_to_vec(&pub_inputs, bincode::config::standard()).unwrap();

    // This should be reworked
    // Public inputs shouldn't be stored in the proof if the verifier wants to check them

    // An u32 is enough for storing proofs up to 32 GiB
    // They shouldn't exceed the order of kbs
    // Reading an usize leads to problem in WASM (32 bit vs 64 bit architecture)

    bytes.extend((proof_bytes.len() as u32).to_le_bytes());
    bytes.extend(proof_bytes);
    bytes.extend(pub_inputs_bytes);

    let Ok(()) = std::fs::write(&proof_path, prefix_hex::encode(bytes)) else {
        eprintln!("Error writing proof to file: {}", &proof_path);
        return;
    };

    println!("Proof written to {}", &proof_path);
}
