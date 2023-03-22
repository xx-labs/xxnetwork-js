use w_ots::{
    hasher::{Blake2bHasher, Sha3_224Hasher},
    keys::Key,
    security::{
        level_0_params, level_1_params,
        level_2_params, level_3_params,
        verify_no_consensus,
    },
};
use wasm_bindgen::prelude::*;
use bip32::XPrv;
use bip39::{Mnemonic, Seed, Language};

/// The following functions use account index 0 and the default WOTS+ parameters (level 0)
/// For more control over derivation see the functions
/// `ext_advanced_wots_get_pk`
/// `ext_advanced_wots_get_pk_from_phrase`
/// `ext_advanced_wots_sign`

/// Get the public key of the WOTS+ key derived from a given BIP39 seed and nonce
#[wasm_bindgen]
pub fn ext_wots_get_pk(seed: &[u8], nonce: u32) -> Vec<u8> {
    // Generate WOTS+ key and get public key
    generate_wots_key_from_seed(seed, 0, 0, nonce).public_key
}

/// Get the public key of the WOTS+ key derived from a given BIP39 phrase and nonce
#[wasm_bindgen]
pub fn ext_wots_get_pk_from_phrase(phrase: &str, nonce: u32) -> Vec<u8> {
    // Get BIP39 seed from mnemonic phrase
    let seed = get_seed_from_phrase(phrase);
    // Generate WOTS+ key and get public key
    generate_wots_key_from_seed(&seed, 0, 0, nonce).public_key
}

/// Sign data using the WOTS+ key derived from a given BIP39 seed and nonce
#[wasm_bindgen]
pub fn ext_wots_sign(seed: &[u8], nonce: u32, data: &[u8]) -> Vec<u8> {
    let key = generate_wots_key_from_seed(seed, 0, 0, nonce);
    match key.sign(data) {
        Ok(sig) => sig,
        _ => panic!("Couldn't sign data.")
    }
}

/// Get the public key of the WOTS+ key derived from a given BIP39 seed
/// for the given account index, security level and nonce
#[wasm_bindgen]
pub fn ext_advanced_wots_get_pk(seed: &[u8], account: u32, level: u32, nonce: u32) -> Vec<u8> {
    // Generate WOTS+ key and get public key
    generate_wots_key_from_seed(seed, account, level, nonce).public_key
}

/// Get the public key of the WOTS+ key derived from a given BIP39 phrase and nonce
#[wasm_bindgen]
pub fn ext_advanced_wots_get_pk_from_phrase(phrase: &str, account: u32, level: u32, nonce: u32) -> Vec<u8> {
    // Get BIP39 seed from mnemonic phrase
    let seed = get_seed_from_phrase(phrase);
    // Generate WOTS+ key and get public key
    generate_wots_key_from_seed(&seed, account, level, nonce).public_key
}

/// Sign message using the WOTS+ key derived from a given BIP39 seed and nonce
#[wasm_bindgen]
pub fn ext_advanced_wots_sign(seed: &[u8], account: u32, level: u32, nonce: u32, message: &[u8]) -> Vec<u8> {
    let key = generate_wots_key_from_seed(seed, account, level, nonce);
    match key.sign(message) {
        Ok(sig) => sig,
        _ => panic!("Couldn't sign data.")
    }
}

/// Verify a WOTS+ signature against the data and public key
#[wasm_bindgen]
pub fn ext_wots_verify(signature: &[u8], message: &[u8], pubkey: &[u8]) -> bool {
    verify_no_consensus(message, signature, pubkey).is_ok()
}

/// Generate a wots key from BIP39 seed with a BIP32 path with given
/// account index, security level and nonce
pub fn generate_wots_key_from_seed(seed: &[u8], account: u32, level: u32, nonce: u32) -> Key::<Blake2bHasher, Sha3_224Hasher> {
    // Apply BIP32 derivation
    let (secret_key, code) = derive_bip32(seed, account, level, nonce);
    // Generate WOTS+ key with given security level and seeds
    generate_wots_key(level, secret_key, code)
}

// Utiltiy functions

/// Get BIP39 seed from given BIP39 phrase
pub fn get_seed_from_phrase(phrase: &str) -> Vec<u8> {
    match Mnemonic::from_phrase(phrase, Language::English) {
        Ok(m) => Seed::new(&m, "").as_bytes().to_vec(),
        _ => panic!("Invalid phrase provided.")
    }
}

/// Apply BIP32 derivation (account index, security level, nonce) to a given BIP39 seed
/// and return the private key and chain code
pub fn derive_bip32(seed: &[u8], account: u32, level: u32, nonce: u32) -> ([u8; 32], [u8; 32]) {
    let path = std::format!("m/44'/1955'/{}'/{}'/{}'", account, level, nonce);
    let ext = XPrv::derive_from_path(seed, &path.parse().unwrap()).unwrap();
    let mut private_key = [0u8; 32];
    private_key.copy_from_slice(&ext.private_key().to_bytes());
    (private_key, ext.attrs().chain_code)
}

/// Generate a WOTS+ key with given security level and seeds
pub fn generate_wots_key(level: u32, seed: [u8; 32], pseed: [u8; 32]) -> Key::<Blake2bHasher, Sha3_224Hasher> {
    let params =  match level {
        0 => level_0_params(),
        1 => level_1_params(),
        2 => level_2_params(),
        3 => level_3_params(),
        _ => panic!("Invalid WOTS+ security level.")
    };
    Key::<Blake2bHasher, Sha3_224Hasher>::from_seed(params, seed, pseed).unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
	fn wots_test_vector() {
		let phrase = "hole define scout taxi help project army vocal sudden wealth volume fan pigeon raven hen spoil cup because crowd wage awkward public reform pluck";
		let expected_public_key =
            hex::decode("7bd49cdc5f70766c70c973a2d6c76b964333ac853c5ae8ecbfef5f1fde08705a")
            .unwrap();
        let public_key = ext_wots_get_pk_from_phrase(phrase, 0);
		assert_eq!(public_key, expected_public_key);
	}
}
