# @xxnetwork/wasm

Various WASM wrappers around xx network specific Rust crates

## overview

This package is a fork of [@polkadot-js/wasm](https://github.com/polkadot-js/wasm), modified to only include extra xx network specific WASM functions.

The main package that should be imported is [wasm-crypto](packages/wasm-crypto/), which contains Sleeve wallet generation functionality.

## building

Packages are built using `@polkadot/dev` tools, which expect all names to start with `@polkadot`.

This means that all the `package.json` files are named using `@polkadot/wasm-*`. This is necessary to make any of the build scripts work properly.

After packages are built, a `post-build.sh` script modifies references from `@polkadot/wasm-*` to `@xxnetwork/wasm-*`. These packages can then be published using NPM to the `@xxnetwork` project. Copyright headers are not modified so that references to the original `@polkadot-js/wasm` project remain intact.