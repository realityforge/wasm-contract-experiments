# Rust/Wasm Experiments on Parity

This repository contains some experiments using Rust compiled to wasm and deployed on the
Kovan Ethereum test network using Parity.

The commands to setup environment:

    $ rustup install nightly
    $ rustup target add wasm32-unknown-unknowns
    $ cargo install -f pwasm-utils

    # add the formatting tool
    $ rustup component add rustfmt-preview
    $ rustup component add rustfmt-preview --toolchain nightly

The commands to build:

    $ cargo build --release --target wasm32-unknown-unknown
    $ wasm-build --target=wasm32-unknown-unknown ./target wasm-contract-experiments

To format code according to rust standard use:

    $ cargo +nightly fmt

# References

* See `vendor/pwasm-tutorial` for the tutorial used to drive the experimentation.
