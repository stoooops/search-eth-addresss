// BIP-39: Generate a mnemonic phrase.
// A random entropy source is used to generate a mnemonic phrase (e.g., "ocean canyon critic kitten coffee measure umbrella tumble ice seminar umbrella").

// BIP-39: Convert the mnemonic phrase into a binary seed.
// The mnemonic phrase is processed through a key derivation function (e.g., PBKDF2 with HMAC-SHA512) to generate a binary seed.

// BIP-32: Use the binary seed as the master seed for an HD wallet.
// The binary seed generated in step 2 is used as the master seed for a BIP-32 HD wallet.

// BIP-32: Generate key pairs from the master seed.
// Using the BIP-32 structure and process, multiple key pairs (public and private keys) are generated from the master seed in a hierarchical and deterministic manner.

use bip32::{
    secp256k1::{
        ecdsa::{SigningKey, VerifyingKey},
        EncodedPoint,
    },
    ExtendedPrivateKey, ExtendedPublicKey,
};
// use hdwallet::rand_core;
// use rand::RngCore;
use num_bigint::BigInt;
use num_traits::One;
use tiny_keccak::Hasher;

// Helper function to convert a public key to an Ethereum address
fn eth_pubkey_to_address(pubkey: &[u8]) -> String {
    let hashed = keccak256(pubkey);
    println!("Keccak256    : 0x{}", hex::encode(hashed));

    // Take the last 20 bytes as the Ethereum address
    let address_bytes = &hashed[12..];
    println!("Address      : 0x{}", hex::encode(address_bytes));

    // Convert to hexadecimal representation
    format!("0x{}", hex::encode(address_bytes))
}

fn bip32_example() -> Result<(), bip32::Error> {
    use bip32::{Mnemonic, XPrv};
    // use rand_core::OsRng;

    let phrase = "dust royal enter exhaust hand hood fork tree flush goat iron rookie job power gold remember small luxury raw broccoli access helmet left fame";

    // Generate random Mnemonic using the default language (English)
    // let mnemonic = Mnemonic::random(&mut OsRng, Default::default());
    let mnemonic = Mnemonic::new(phrase, Default::default())?;
    println!("Phrase       : {}", mnemonic.phrase());

    // Derive a BIP39 seed value using the given password
    let seed = mnemonic.to_seed("");
    println!("Seed         : 0x{}", hex::encode(&seed));

    // Derive the root `XPrv` from the `seed` value
    // let root_xprv = XPrv::new(&seed)?;
    // assert_eq!(root_xprv, XPrv::derive_from_path(&seed, &"m".parse()?)?);

    // let root_xpub = root_xprv.public_key();
    // println!("Root XPub: {}", root_xpub.to_string(Prefix::XPUB));

    // Derive a child `XPrv` using the provided BIP32 derivation path
    let child_path = "m/44'/60'/0'/0/0";

    let child_xprv: ExtendedPrivateKey<SigningKey> =
        XPrv::derive_from_path(&seed, &child_path.parse()?)?;
    let signing_key: &SigningKey = child_xprv.private_key();
    println!("Private Key  : 0x{}", hex::encode(signing_key.to_bytes()));

    // Get the `XPub` associated with `child_xprv`.
    let child_xpub: ExtendedPublicKey<VerifyingKey> = child_xprv.public_key();
    let verification_key: &VerifyingKey = child_xpub.public_key();
    // println!(
    //     "Public Key   : 0x{}",
    //     hex::encode(verification_key.to_bytes()),
    // );
    // should be 33 bytes
    // assert_eq!(verification_key.to_bytes().len(), 33);
    // let binding = EncodedPoint::from(verification_key);
    // let public_key_bytes: &[u8] = binding.as_bytes();
    // println!("Public Key   : 0x{}", hex::encode(public_key_bytes));

    let uncompressed_pubkey = decompress_pubkey(&verification_key.to_bytes());
    println!("Uncompressed : 0x{}", hex::encode(&uncompressed_pubkey));

    eth_pubkey_to_address(&uncompressed_pubkey);
    // println!("Address      : {}", address);

    Ok(())
}

// reference implementation from python:
// def uncompressed(self, compressed: Optional[str] = None) -> str:
// """
// Get Uncommpresed Public Key.

// :param compressed: Compressed public key, default to ``None``.
// :type compressed: str

// :returns: str -- Uncommpresed public key.

// >>> from hdwallet import HDWallet
// >>> from hdwallet.symbols import BTC
// >>> hdwallet = HDWallet(symbol=BTC)
// >>> hdwallet.from_mnemonic(mnemonic="venture fitness paper little blush april rigid where find volcano fetch crack label polar dash")
// >>> hdwallet.from_path(path="m/44'/0'/0'/0/0")
// >>> hdwallet.uncompressed()
// "f93f58b97c3bb616645c3dda256ec946d87c45baf531984c022dd0fd1503b0a875f63285a539213ac241fc4a88e7137ba1c8d897b1c1e5efb81bfc6b45a22d40"
// """

// p = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F
// public_key = unhexlify(compressed) if compressed else unhexlify(self.compressed())
// x = int.from_bytes(public_key[1:33], byteorder='big')
// y_sq = (pow(x, 3, p) + 7) % p
// y = pow(y_sq, (p + 1) // 4, p)
// if y % 2 != public_key[0] % 2:
//     y = p - y
// y = y.to_bytes(32, byteorder='big')
// return (public_key[1:33] + y).hex()

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

fn main() {
    // BIP-32: Use the binary seed as the master seed for an HD wallet.
    bip32_example().unwrap();
}
