### Week 2 of RUST

Create contract as lib 
```
cargo new --lib project-name
```

adding few dependency on Cargo.toml

### build
build the contract for deployment

```
cargo build --release --target wasm32-unknown-unknown
```

### Deploy with gnerating account 

```
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/near_sc_1.wasm
```

https://explorer.testnet.near.org/transactions/2gf4MYirNPzxgFzx8e8iSLUdqeLWwdX2JwUXa4gLZQZZ