// Copyright 2019-2022 @polkadot/wasm-crypto authors & contributors
// SPDX-License-Identifier: Apache-2.0

import { assert } from '@polkadot/util';
import { it } from 'node:test';

export function sleeveTestVector (wasm) {
  it('creates a known phrase using sleeve from a known mnemonic', () => {
    const mnemomic = "hamster diagram private dutch cause delay private meat slide toddler razor book happy fancy gospel tennis maple dilemma loan word shrug inflict delay length";
    const expected = "speed bar erosion clog exist siren giraffe liar sick hire lazy disagree pig monitor loan owner solve grant excess drop broom render roast primary";
    const res = wasm.generateSleeve(mnemomic);

    assert(expected === res, 'ERROR: Sleeve test vector failed');
  });
}
