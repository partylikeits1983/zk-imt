use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let root_path = Path::new("../data/root.txt");
    let commitmenthash_path = Path::new("../data/commitment_hash.txt");
    let nulifierhash_path = Path::new("../data/nullifier_hash.txt");
    let siblings_path = Path::new("../data/proof_siblings.txt");
    let indices_path = Path::new("../data/proof_path_indices.txt");
    let nulifier_path = Path::new("../data/nullifier.txt");
    let secret_path = Path::new("../data/secret.txt");

    // Edit to have two Prover.toml files
    let withdraw_output_path = Path::new("../circuits/imt/Prover.toml");

    // Open the output file for writing, create it if it doesn't exist, or truncate it if it does
    let mut withdraw_output_path = File::create(&withdraw_output_path)?;

    // Initialize variables for leaf and root values
    let commitmenthash_value = read_first_line(&commitmenthash_path)?;
    let nulifierhash_value = read_first_line(&nulifierhash_path)?;
    let root_value = read_first_line(&root_path)?;
    let nulifier_value = read_first_line(&nulifier_path)?;
    let secret_value = read_first_line(&secret_path)?;

    // withdraw/Prover.toml
    writeln!(withdraw_output_path, "commitment_hash = \"{}\"", commitmenthash_value)?;
    writeln!(withdraw_output_path, "nullifier_hash = \"{}\"", nulifierhash_value)?;
    writeln!(withdraw_output_path, "root = \"{}\"", root_value)?; 
    writeln!(withdraw_output_path, "nullifier = \"{}\"", nulifier_value)?;  
    writeln!(withdraw_output_path, "secret = \"{}\"", secret_value)?;  
    
    writeln!(withdraw_output_path, "proof_siblings = [")?;
    for line in io::BufReader::new(File::open(&siblings_path)?).lines() {
        let line = line?; // Handle potential errors in line reading
        writeln!(withdraw_output_path, "    \"{}\",", line)?;
    }
    writeln!(withdraw_output_path, "]\n")?; // Add a newline for readability

    // Write proofPathIndices array
    writeln!(withdraw_output_path, "proof_path_indices = [")?;
    for line in io::BufReader::new(File::open(&indices_path)?).lines() {
        let line = line?; // Handle potential errors in line reading
        writeln!(withdraw_output_path, "    {},", line)?;
    }
    writeln!(withdraw_output_path, "]")?;

    Ok(())
}

// Helper function to read the first line from a file
fn read_first_line(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = io::BufReader::new(file);
    let mut line = String::new();
    // Only read the first line
    buf_reader.read_line(&mut line)?;
    // Trim the newline character(s) at the end of the line
    Ok(line.trim_end().to_string())
}
