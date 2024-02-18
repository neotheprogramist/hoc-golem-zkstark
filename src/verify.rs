use platinum_prover::air::verify_cairo_proof;
use stark_platinum_prover::proof::options::{ProofOptions, SecurityLevel};

// reference https://github.com/lambdaclass/lambdaworks/blob/main/provers/cairo/src/main.rs
pub fn verify(proof: Vec<u8>) -> bool {
    let proof_options = ProofOptions::new_secure(SecurityLevel::Conjecturable100Bits, 3);

    let mut bytes = proof.as_slice();
    if bytes.len() < 8 {
        eprintln!("Error reading proof");
        return false;
    }

    // Proof len was stored as an u32, 4u8 needs to be read
    let proof_len = if let Some(x) = bytes.get(0..4) {
        if let Ok(x) = x.try_into() {
            u32::from_le_bytes(x) as usize
        } else {
            eprintln!("Error reading proof");
            return false;
        }
    } else {
        eprintln!("Error reading proof");
        return false;
    };

    bytes = &bytes[4..];
    if bytes.len() < proof_len {
        eprintln!("Error reading proof");
        return false;
    }

    let Ok((proof, _)) =
        bincode::serde::decode_from_slice(&bytes[0..proof_len], bincode::config::standard())
    else {
        println!("Error reading proof");
        return false;
    };
    bytes = &bytes[proof_len..];

    let Ok((pub_inputs, _)) = bincode::serde::decode_from_slice(bytes, bincode::config::standard())
    else {
        println!("Error reading proof");
        return false;
    };

    verify_cairo_proof(&proof, &pub_inputs, &proof_options)
}
