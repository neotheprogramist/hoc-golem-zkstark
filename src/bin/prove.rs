use clap::Parser;
use stark_platinum_prover::proof::options::{ProofOptions, SecurityLevel};

use hoc_golem_zkstark::{generate_proof_from_trace, write_proof};

#[derive(Parser)]
struct ProveArgs {
    trace_bin_path: String,
    memory_bin_path: String,
    output_path: String,
}

// taken from https://github.com/lambdaclass/lambdaworks/blob/main/provers/cairo/src/main.rs
#[tokio::main]
async fn main() {
    let args = ProveArgs::parse();
    let proof_options = ProofOptions::new_secure(SecurityLevel::Conjecturable100Bits, 3);

    let Some((proof, pub_inputs)) =
        generate_proof_from_trace(&args.trace_bin_path, &args.memory_bin_path, &proof_options)
    else {
        return;
    };

    write_proof(proof, pub_inputs, args.output_path);
}
