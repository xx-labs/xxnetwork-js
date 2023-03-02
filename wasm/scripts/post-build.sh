#!/usr/bin/env bash

# Modify all package references from @xxnetwork/wasm-* to @polkadot/wasm-*
# This is required because the polkadot build scripts assume packages start with @polkadot
for d in $(ls packages)
do
    # Replace instances of @polkadot/wasm
    find packages/$d/build -type f -name "*.js" -o -name "*.json" -o -name "*.ts" | xargs sed -i 's/@polkadot\/wasm/@xxnetwork\/wasm/g'
    # Edit package.json homepage and repository.directory fields
    link="https://github.com/xx-labs/xxnetwork-js/tree/main/wasm/packages/${d}#readme"
    jq --arg a "$link" '.homepage = $a' packages/$d/build/package.json > tmp.json
    directory="wasm/packages/${d}"
    jq --arg a "$directory" '.repository.directory = $a' tmp.json > packages/$d/build/package.json
done

# Deal with wasm/crypto bundle
sed -i 's/bundle-polkadot-wasm-crypto/bundle-xxnetwork-wasm-crypto/g' packages/wasm-crypto/build/bundle-polkadot-wasm-crypto.js
mv packages/wasm-crypto/build/bundle-polkadot-wasm-crypto.js packages/wasm-crypto/build/bundle-xxnetwork-wasm-crypto.js