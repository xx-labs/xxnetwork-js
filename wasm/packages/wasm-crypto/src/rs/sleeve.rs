use w_ots::{
    hasher::{Blake2bHasher, Sha3_224Hasher, Sha3_256Hasher, Hasher},
    keys::Key,
    security::level_0_params,
};
use bip32::XPrv;
use wasm_bindgen::prelude::*;
use bip39::{Mnemonic, Seed, Language};

/// Generate a Sleeve wallet
#[wasm_bindgen]
pub fn ext_generate_sleeve(phrase: &str) -> String {
    // Default path
    let path = "m/44'/1955'/0'/0'/0'";
    // Get master seed from mnemonic phrase
    let seed = match Mnemonic::from_phrase(phrase, Language::English) {
		Ok(m) => Seed::new(&m, "").as_bytes().to_vec(),
		_ => panic!("Invalid phrase provided.")
	};
    // Derive BIP32 private key
    let ext = XPrv::derive_from_path(&seed, &path.parse().unwrap()).unwrap();
    let mut secret_key = [0u8; 32];
    secret_key.copy_from_slice(&ext.private_key().to_bytes());
    let code = ext.attrs().chain_code;
    // Generate WOTS+ key with default parameters
    let params = level_0_params();
    let key = Key::<Blake2bHasher, Sha3_224Hasher>::from_seed(params, secret_key, code).unwrap();
    // Derive Sleeve secret key
    let mut secret = [0u8; 32];
    let mut hasher = Sha3_256Hasher::new();
    hasher.write(b"xx network sleeve".to_vec());
    hasher.write(secret_key.to_vec());
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
