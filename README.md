# zk-imt simplified

running:
```
nargo execute witness
```

Prove an execution of the Noir program
```
bb prove -b ./target/hash_preimage.json -w ./target/witness.gz -o ./target/proof
```

Verify the execution proof
```
bb write_vk -b ./target/hash_preimage.json -o ./target/vk
```

Generate Solidity verifier:
```
bb write_vk -b ./target/hash_preimage.json
bb contract
```