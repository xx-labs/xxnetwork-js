// Copyright 2019-2022 @polkadot/wasm-crypto authors & contributors
// SPDX-License-Identifier: Apache-2.0

const { assert } = require('@polkadot/util');

const sleeve = require('./sleeve.js');

const tests = {
  ...sleeve
};

async function beforeAll (name, wasm) {
  const result = await wasm.waitReady();

  console.log(`*** waitReady()=${result} for ${wasm.bridge.type}`);

  assert(name.toLowerCase() === wasm.bridge.type, `Incorrect environment launched, expected ${name.toLowerCase()}, found ${wasm.bridge.type}`);

  return result;
}

function runAll (name, wasm) {
  const failed = [];
  let count = 0;

  Object
    .entries(tests)
    .forEach(([name, test]) => {
      const timerId = `\t${name}`;

      count++;

      try {
        console.time(timerId);
        console.log();
        console.log(timerId);

        test(wasm);

        console.timeEnd(timerId);
      } catch (error) {
        console.error();
        console.error(error);

        failed.push(name);
      }
    });

  if (failed.length) {
    throw new Error(`\n*** ${name}: FAILED: ${failed.length} of ${count}: ${failed.join(', ')}`);
  }
}

function runUnassisted (name, wasm) {
  console.log(`\n*** ${name}: Running tests`);

  beforeAll(name, wasm)
    .then(() => runAll(name, wasm))
    .then(() => {
      console.log(`\n*** ${name}: All passed`);
      process.exit(0);
    })
    .catch((error) => {
      console.error(error.message, '\n');
      process.exit(-1);
    });
}

module.exports = {
  beforeAll,
  runAll,
  runUnassisted,
  tests
};
