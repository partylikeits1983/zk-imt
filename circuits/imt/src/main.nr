use dep::std::hash::poseidon;

fn main(
    root: pub Field,
    nullifier_hash: pub Field,
    nullifier: Field,
    secret: Field,
    proof_siblings: [Field; 32],
    proof_path_indices: [Field; 32]
) {
    let leaf = poseidon::bn254::hash_2([nullifier, secret]);

    assert(nullifier_hash == poseidon::bn254::hash_1([nullifier]));

    let mut hash: Field = leaf;

    for i in 0..32 {
        if proof_path_indices[i] == 0 {
            let input: [Field; 2] = [hash, proof_siblings[i]];
            hash = poseidon::bn254::hash_2(input);
        } else {
            let input: [Field; 2] = [proof_siblings[i], hash];
            hash = poseidon::bn254::hash_2(input);
        }
    }
    assert(hash == root);
}

#[test]
fn test_main() {
    // Step 1: Set up inputs
    let nullifier = 123456789; // Treated as Field element
    let secret = 987654321; // Treated as Field element

    // Step 2: Compute the leaf hash
    let leaf = poseidon::bn254::hash_2([nullifier, secret]);

    // Step 3: Compute the nullifier hash
    let nullifier_hash = poseidon::bn254::hash_1([nullifier]);

    // Step 4: Generate Merkle proof (all siblings are zeros)
    let proof_siblings: [Field; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let proof_path_indices: [Field; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // All zeros (left child)

    // Step 5: Compute the root hash
    let mut hash = leaf;
    for i in 0..32 {
        if proof_path_indices[i] == 0 {
            // Current node is a left child
            hash = poseidon::bn254::hash_2([hash, proof_siblings[i]]);
        } else {
            // Current node is a right child
            hash = poseidon::bn254::hash_2([proof_siblings[i], hash]);
        }
    }
    let root = hash;

    println(root);
    println(nullifier_hash);
    println(nullifier);
    println(secret);
    println(proof_siblings);
    println(proof_path_indices);

    // Step 6: Invoke the main function
    main(
        root,
        nullifier_hash,
        nullifier,
        secret,
        proof_siblings,
        proof_path_indices
    );
}
