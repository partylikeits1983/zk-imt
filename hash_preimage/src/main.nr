use dep::std::hash::poseidon;

fn main(hash: pub Field, hash_preimage: Field) {
    let internal_hash = poseidon::bn254::hash_1([hash_preimage]);

    assert(internal_hash == hash);
}

#[test]
fn test_main() {
    let hash_preimage = 0;
    let hash = poseidon::bn254::hash_1([hash_preimage]);

    main(hash, hash_preimage);
}
