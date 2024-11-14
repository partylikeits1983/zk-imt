use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Number of public inputs (including return values)
    let num_public_inputs = 1; // Change this value as needed
    let public_input_bytes = 32 * num_public_inputs;

    // Read the proof file
    let mut file = File::open("../hash_preimage/target/proof")?;
    let mut proof_data = Vec::new();
    file.read_to_end(&mut proof_data)?;

    if proof_data.len() < public_input_bytes {
        eprintln!("Error: Proof file is smaller than expected public input size.");
        std::process::exit(1);
    }

    // Extract public inputs
    let public_inputs_bytes = &proof_data[..public_input_bytes];

    // Split public inputs into 32-byte chunks
    let public_inputs: Vec<String> = public_inputs_bytes
        .chunks(32)
        .map(|chunk| format!("0x{}", hex::encode(chunk)))
        .collect();

    // Extract proof (the rest of the file)
    let proof_bytes = &proof_data[public_input_bytes..];
    let proof_hex = format!("0x{}", hex::encode(proof_bytes));

    // Print the public inputs and proof
    println!("Public inputs:");
    for (idx, input_hex) in public_inputs.iter().enumerate() {
        println!("Input {}: {}", idx, input_hex);
    }

    println!("\nProof:");
    println!("{}", proof_hex);

    Ok(())
}
