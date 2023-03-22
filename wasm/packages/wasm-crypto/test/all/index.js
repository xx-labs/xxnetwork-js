// Copyright 2019-2023 @polkadot/wasm-crypto authors & contributors
// SPDX-License-Identifier: Apache-2.0

import { assert } from '@polkadot/util';

import * as sleeve from './sleeve.js';

export const tests = {
  sleeve
};

export async function initRun (name, wasm) {
  const result = await wasm.waitReady();

  console.log(`*** waitReady()=${result} for ${wasm.bridge.type}`);

  assert(name.toLowerCase() === wasm.bridge.type, `Incorrect environment launched, expected ${name.toLowerCase()}, found ${wasm.bridge.type}`);

  return result;
}

export function runAll (name, wasm) {
  const failed = [];
  let count = 0;

  Object
    .entries(tests)
    .forEach(([describeName, tests]) => {
      describe(describeName, () => {
        Object
          .entries(tests)
          .forEach(([name, test]) => {
            const timerId = `\t${name}`;

            count++;

            try {
              console.time(timerId);
              console.log();
              // console.log(timerId);

              test(wasm);

              console.timeEnd(timerId);
            } catch (error) {
              console.error();
              console.error(error);

              failed.push(name);
            }
          });
      });
    });

  if (failed.length) {
    throw new Error(`\n*** ${name}: FAILED: ${failed.length} of ${count}: ${failed.join(', ')}`);
  }
}

export function runUnassisted (type, wasm) {
  console.log(`\n*** ${type}: Running tests`);

  // for these we are pass-through describe and it handlers
  globalThis.describe = (name, fn) => {
    console.log('\n', name);

    // eslint-disable-next-line @typescript-eslint/no-floating-promises
    fn();
  };

  globalThis.it = (name, fn) => {
    console.log(`\t${name}`);

    // eslint-disable-next-line @typescript-eslint/no-floating-promises
    fn();
  };

  console.time(type);

  initRun(type, wasm)
    .then(() => {
      runAll(type, wasm);
      console.log(`\n*** ${type}: All passed`);
      console.timeEnd(type);
      console.log();
      process.exit(0);
    })
    .catch((error) => {
      console.error(`\n*** ${type}: FAILED:`, error.message, '\n');
      console.timeEnd(type);
      console.log();
      process.exit(-1);
    });
}
