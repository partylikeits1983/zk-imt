# ZK-IMT


Generate Circuit Data:
```
forge test 
```

format Prover.toml file
```
cd format_imt_prover
cargo run
```

### Generate Solidity Verifier:

Prove an execution of the Noir program
```
cd circuits/imt
bb prove -b ./target/imt.json -w ./target/imt.gz -o ./target/proof
```


Verify the execution proof
```
bb write_vk -b ./target/imt.json -o ./target/vk
```

Generate Solidity verifier:
```
bb write_vk -b ./target/imt.json
bb contract
``` 