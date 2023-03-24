use bip32::{
    secp256k1::ecdsa::{SigningKey, VerifyingKey},
    ExtendedPrivateKey, ExtendedPublicKey, Language, Mnemonic, Seed, XPrv,
};
use num_bigint::BigInt;
use num_traits::One;
use tiny_keccak::Hasher;

use crate::randnum::Entropy;

pub trait AddressGenerator {
    fn generate(&self, entropy: Entropy) -> Result<String, bip32::Error>;
}

pub struct MnemonicAddressGenerator {
    pub language: Language,
}

impl AddressGenerator for MnemonicAddressGenerator {
    fn generate(&self, entropy: Entropy) -> Result<String, bip32::Error> {
        // Generate random Mnemonic using the specified language
        let mnemonic = Mnemonic::from_entropy(entropy, self.language);

        // Derive a BIP39 seed value using the empty password
        let seed: Seed = mnemonic.to_seed("");

        // Derive Ethereum address from seed
        let child_xprv: ExtendedPrivateKey<SigningKey> =
            XPrv::derive_from_path(&seed, &"m/44'/60'/0'/0/0".parse()?)?;
        let child_xpub: ExtendedPublicKey<VerifyingKey> = child_xprv.public_key();
        let verifying_key: &VerifyingKey = child_xpub.public_key();
        let uncompressed_pubkey = decompress_pubkey(&verifying_key.to_bytes());

        let mut hashed_pubkey = [0u8; 32];
        let mut keccak = tiny_keccak::Keccak::v256();
        keccak.update(&uncompressed_pubkey);
        keccak.finalize(&mut hashed_pubkey);

        let address_bytes = &hashed_pubkey[12..];
        let address = format!("0x{}", hex::encode(address_bytes));

        Ok(address)
    }
}

// reference implementation from python hdwallet:
//
//   ```python
//   p = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F
//   public_key = unhexlify(compressed) if compressed else unhexlify(self.compressed())
//   x = int.from_bytes(public_key[1:33], byteorder='big')
//   y_sq = (pow(x, 3, p) + 7) % p
//   y = pow(y_sq, (p + 1) // 4, p)
//   if y % 2 != public_key[0] % 2:
//       y = p - y
//   y = y.to_bytes(32, byteorder='big')
//   return (public_key[1:33] + y).hex()
//  ```
//
// Rust implementation:
fn decompress_pubkey(compressed_pubkey: &[u8]) -> Vec<u8> {
    // p = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F
    let p = BigInt::parse_bytes(
        b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        16,
    )
    .unwrap();

    // x = int.from_bytes(public_key[1:33], byteorder='big')
    let x: BigInt = BigInt::from_bytes_be(num_bigint::Sign::Plus, &compressed_pubkey[1..33]);

    // y_sq = (pow(x, 3, p) + 7) % p
    let y_sq: BigInt = (x.clone().modpow(&BigInt::from(3), &p) + 7) % &p;

    // y = pow(y_sq, (p + 1) // 4, p)
    let y_exp = (p.clone() + BigInt::one()) / 4;
    let y: BigInt = y_sq.modpow(&y_exp, &p);

    // if y % 2 != public_key[0] % 2:
    //     y = p - y
    let mut y_bytes = y.to_bytes_be().1;
    if y.clone() % 2 != (compressed_pubkey[0] % 2).into() {
        y_bytes = (p.clone() - y).to_bytes_be().1;
    }

    // y = y.to_bytes(32, byteorder='big')
    // return (public_key[1:33] + y).hex()
    [&compressed_pubkey[1..33], &y_bytes].concat()
}

// pub struct MnemonicAddress {
//     pub mnemonic: Mnemonic,
//     pub address: String,
// }

// pub fn generate(entropy: Entropy) -> Result<MnemonicAddress, bip32::Error> {
//     // let phrase = "dust royal enter exhaust hand hood fork tree flush goat iron rookie job power gold remember small luxury raw broccoli access helmet left fame";
//     // Self::from_entropy(entropy, language)

//     // Generate random Mnemonic using the default language (English)
//     let mnemonic = Mnemonic::from_entropy(entropy, Default::default());
//     // let mnemonic: Mnemonic = Mnemonic::new(phrase, Default::default())?;

//     // Derive a BIP39 seed value using the given password
//     let seed: Seed = mnemonic.to_seed("");

//     // private key
//     let child_xprv: ExtendedPrivateKey<SigningKey> =
//         XPrv::derive_from_path(&seed, &"m/44'/60'/0'/0/0".parse()?)?;

//     // public key
//     let child_xpub: ExtendedPublicKey<VerifyingKey> = child_xprv.public_key();
//     let verifying_key: &VerifyingKey = child_xpub.public_key();
//     let uncompressed_pubkey = decompress_pubkey(&verifying_key.to_bytes());

//     // keccak256
//     let mut hashed_pubkey = [0u8; 32];
//     let mut keccak = tiny_keccak::Keccak::v256();
//     keccak.update(&uncompressed_pubkey);
//     keccak.finalize(&mut hashed_pubkey);

//     // Take the last 20 bytes as the Ethereum address
//     let address_bytes = &hashed_pubkey[12..];

//     // address
//     let address = format!("0x{}", hex::encode(address_bytes));

//     Ok(MnemonicAddress { mnemonic, address })
// }
