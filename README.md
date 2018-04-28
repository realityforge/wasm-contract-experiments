# Rust/Wasm Experiments on Parity

This repository contains some experiments using Rust compiled to wasm and deployed on the
Kovan Ethereum test network using Parity.

The commands to setup environment:

    $ rustup install nightly
    $ rustup target add wasm32-unknown-unknowns
    $ cargo install -f pwasm-utils

The commands to build:

    $ cargo build --release --target wasm32-unknown-unknown
    $ wasm-build --target=wasm32-unknown-unknown ./target wasm-contract-experiments

# References

* See `vendor/pwasm-tutorial` for the tutorial used to drive the experimentation.
