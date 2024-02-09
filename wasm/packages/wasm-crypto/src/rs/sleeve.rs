use w_ots::hasher::{Sha3_256Hasher, Hasher};
use wasm_bindgen::prelude::*;
use bip39::{Mnemonic, Language};
use crate::wots::{get_seed_from_phrase, derive_bip32, generate_wots_key};

/// Generate a Sleeve wallet
#[wasm_bindgen]
pub fn ext_generate_sleeve(phrase: &str) -> String {
    // Get BIP39 seed from mnemonic phrase
    let seed = get_seed_from_phrase(phrase);
    // Apply BIP32 derivation with default path (0, 0, 0)
    let (priv_key, code) = derive_bip32(&seed, 0, 0, 0);
    // Generate WOTS+ key from seeds
    return generate_sleeve_from_seed(priv_key, code);
}

/// Generate a Sleeve wallet from a given bip32 node
#[wasm_bindgen]
pub fn ext_generate_sleeve_from_node(priv_key: &[u8], code: &[u8]) -> String {
    if priv_key.len() != 32 || code.len() != 32 {
        panic!("Invalid key length.");
    }
    // Copy into arrays
    let mut pk = [0u8; 32];
    pk.copy_from_slice(priv_key);
    let mut cd = [0u8; 32];
    cd.copy_from_slice(code);
    // Generate WOTS+ key from seeds
    return generate_sleeve_from_seed(pk, cd);
}

pub fn generate_sleeve_from_seed(seed: [u8; 32], pseed: [u8; 32]) -> String {
    let key = generate_wots_key(0, seed, pseed);
    // Derive Sleeve secret key
    let mut secret = [0u8; 32];
    let mut hasher = Sha3_256Hasher::new();
    hasher.write(b"xx network sleeve".to_vec());
    hasher.write(seed.to_vec());
    hasher.sum(&mut secret);
    hasher = Sha3_256Hasher::new();
    hasher.write(secret.to_vec());
    hasher.write(key.public_key);
    hasher.sum(&mut secret);
    match Mnemonic::from_entropy(&secret, Language::English) {
        Ok(m) => m.into_phrase(),
        _ => panic!("Couldn't create mnemonic from entropy.")
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
	fn sleeve_test_vector() {
		let phrase = "hamster diagram private dutch cause delay private meat slide toddler razor book happy fancy gospel tennis maple dilemma loan word shrug inflict delay length";
		let expected = "speed bar erosion clog exist siren giraffe liar sick hire lazy disagree pig monitor loan owner solve grant excess drop broom render roast primary";
		let out_mnem = ext_generate_sleeve(phrase);
		assert_eq!(out_mnem, expected);
	}
}
