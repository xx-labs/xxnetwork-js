// Copyright 2019-2023 @polkadot/wasm-crypto-init authors & contributors
// SPDX-License-Identifier: Apache-2.0

import type { WasmBaseInstance } from '@polkadot/wasm-bridge/types';

/* eslint-disable camelcase */

// wasm-pack build output (formatted) from pkg/wasm_bg.d.ts
export interface WasmCryptoInstance extends WasmBaseInstance {
  // exposed functions

  ext_generate_sleeve(resLen: 8, ptrPhrase: number, lenPhrase: number): void;

  ext_wots_get_pk(resLen: 8, ptrSeed: number, lenSeed: number, nonce: number): void;

  ext_wots_get_pk_from_phrase(resLen: 8, ptrPhrase: number, lenPhrase: number, nonce: number): void;

  ext_wots_sign(resLen: 8, ptrSeed: number, lenSeed: number, nonce: number, ptrMsg: number, lenMsg: number): void;

  ext_advanced_wots_get_pk(resLen: 8, ptrSeed: number, lenSeed: number, account: number, level: number, nonce: number): void;

  ext_advanced_wots_get_pk_from_phrase(resLen: 8, ptrPhrase: number, lenPhrase: number, account: number, level: number, nonce: number): void;

  ext_advanced_wots_sign(resLen: 8, ptrSeed: number, lenSeed: number, account: number, level: number, nonce: number, ptrMsg: number, lenMsg: number): void;

  ext_wots_verify(ptrSig: number, lenSig: number, ptrMsg: number, lenMsg: number, ptrPub: number, lenPub: number): number;
}
