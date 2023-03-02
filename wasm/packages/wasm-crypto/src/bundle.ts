// Copyright 2019-2023 @polkadot/wasm-crypto authors & contributors
// SPDX-License-Identifier: Apache-2.0

import type { WasmCryptoInstance } from '@polkadot/wasm-crypto-init/types';

import { bridge, initBridge } from './init';

export { packageInfo } from './packageInfo';
export { bridge };

// Removes the first parameter (expected as WasmCryptoInstance) and leaves the
// rest of the parameters in-tack. This allows us to dynamically create a function
// return from the withWasm helper
type PopFirst<T extends unknown[]> =
  T extends [WasmCryptoInstance, ...infer N]
    ? N
    : [];

/**
 * @internal
 * @description
 * This create an extenal interface function from the signature, all the while checking
 * the actual bridge wasm interface to ensure it has been initialized.
 *
 * This means that we can call it
 *
 *   withWasm(wasm: WasmCryptoInstance, a: number, b: string) => Uint8Array
 *
 * and in this case it will create an interface function with the signarure
 *
 *   (a: number, b: string) => Uint8Array
 */
function withWasm <T, F extends (wasm: WasmCryptoInstance, ...params: never[]) => T> (fn: F): (...params: PopFirst<Parameters<F>>) => ReturnType<F> {
  return (...params: PopFirst<Parameters<F>>): ReturnType<F> => {
    if (!bridge.wasm) {
      throw new Error('The WASM interface has not been initialized. Ensure that you wait for the initialization Promise with waitReady() from @xxnetwork/wasm-crypto before attempting to use WASM-only interfaces.');
    }

    return fn(bridge.wasm, ...params) as ReturnType<F>;
  };
}

export const generateSleeve = withWasm((wasm, phrase: string): string => {
  wasm.ext_generate_sleeve(8, ...bridge.allocString(phrase));

  return bridge.resultString();
});

export function isReady (): boolean {
  return !!bridge.wasm;
}

export async function waitReady (): Promise<boolean> {
  try {
    const wasm = await initBridge();

    return !!wasm;
  } catch {
    return false;
  }
}
