// Copyright 2019-2023 @polkadot/wasm-crypto-init authors & contributors
// SPDX-License-Identifier: Apache-2.0

import type { WasmBaseInstance } from '@polkadot/wasm-bridge/types';

/* eslint-disable camelcase */

// wasm-pack build output (formatted) from pkg/wasm_bg.d.ts
export interface WasmCryptoInstance extends WasmBaseInstance {
  // exposed functions

  ext_generate_sleeve(resLen: 8, ptrPhrase: number, lenPhrase: number): void;
}
