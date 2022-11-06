# Rust Smart Contract Template

## Getting started

1. Set up the [prerequisites](https://github.com/near/near-sdk-rs#pre-requisites)
2. Begin writing your smart contract in `src/lib.rs`
3. Test the contract  

    `cargo test -- --nocapture`

4. Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`

**Get more info at:**

* [Rust Smart Contract Quick Start](https://docs.near.org/develop/prerequisites)
* [Rust SDK Book](https://www.near-sdk.io/)
