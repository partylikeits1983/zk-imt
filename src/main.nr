use dep::std::hash::poseidon;

fn main(
    root: pub Field,
    nulfierHash: pub Field,
    nullifier: Field,
    secret: Field,
    proofSiblings: [Field; 32],
    proofPathIndicies: [Field; 32]
) {
    let leaf = poseidon::bn254::hash_2([nullifier, secret]);

    assert(nulfierHash == poseidon::bn254::hash_1([nullifier]));

    let mut hash: Field = leaf;

    for i in 0..32 {
        if proofPathIndicies[i] == 0 {
            let input: [Field; 2] = [hash, proofSiblings[i]];
            hash = poseidon::bn254::hash_2(input);
        } else {
            let input: [Field; 2] = [proofSiblings[i], hash];
            hash = poseidon::bn254::hash_2(input);
        }
    }
    assert(hash == root);
}

