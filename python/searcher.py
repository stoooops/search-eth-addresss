#!/usr/bin/env python3

"""
Generate an Ethereum public address with a given prefix, encrypt the
corresponding private key with a password using the AES-128-CTR cipher, and
generate a keystore file containing the encrypted private key.

The script uses the Scrypt key derivation function to generate a key and a MAC
from the password and salt, and generates an Initialization Vector (IV) to use
with AES-128-CTR encryption. It then generates a keystore file in JSON format
containing the address, crypto parameters and a unique ID.

Usage:
    python searcher.py --prefix <desired_prefix> --password <password_file> -o <output_directory>

Arguments:
    desired_prefix (str): Desired prefix for the public address.
    password_file (str): Password file to encrypt the private key.
    output_directory (str): Output directory for the keystore file.

Example:
    python searcher.py --prefix 0x000000 --password execution/keystore/password.txt -o execution/keystore
"""

##############################################################################
# Public/Private Key Pair
##############################################################################

import os
from typing import Generator, Optional

from core.address import Address, KeyPair, PrivateKey
from core.keystore import encrypt_key_file
from Crypto.Hash import keccak  # type: ignore
from ecdsa import SECP256k1, SigningKey, VerifyingKey  # type: ignore


def search_for_prefix(prefix: str) -> Generator[KeyPair, None, None]:
    """Search for a public address with the given prefix."""
    if prefix.startswith("0x"):
        prefix = prefix[2:]

    print(f"Searching for public address with prefix 0x{prefix}...")
    prefix_length = len(prefix)
    max_prefix_collisions = 0
    best: Optional[KeyPair] = None

    while max_prefix_collisions < prefix_length:
        # Generate a new random private key
        signing_key: SigningKey = SigningKey.generate(curve=SECP256k1)
        # Get the verifying key from the signing key.
        verifying_key: VerifyingKey = signing_key.get_verifying_key()
        # Convert the uncompressed public key to bytes.
        public_key: bytes = verifying_key.to_string("uncompressed")
        # Compute the Keccak-256 hash of the public key bytes.
        # Take the last 20 bytes of the hash.
        public_key_hash: bytes = keccak.new(
            digest_bits=256, data=public_key[1:]
        ).digest()[-20:]
        # Prepend the string with "0x" to create the Ethereum address.

        prefix_bytes_to_check = public_key_hash[:prefix_length]
        prefix_str_to_check = prefix_bytes_to_check.hex()

        # Compute the number of prefix collisions between the public address and the desired prefix
        commonprefix = os.path.commonprefix([prefix_str_to_check, prefix])

        num_prefix_collisions = len(commonprefix)
        # If the current number of prefix collisions is greater than the current maximum, update the maximum and print it
        if num_prefix_collisions >= max_prefix_collisions:
            pk: PrivateKey = PrivateKey(signing_key.to_string())
            public_address: Address = Address(f"0x{public_key_hash.hex()}")
            key_pair = KeyPair(public_address, pk)

            # check if we found a better match
            if num_prefix_collisions > max_prefix_collisions:
                print("\n" * num_prefix_collisions)  # clear the screen
                print(f"New max prefix collisions: {num_prefix_collisions}")
                print("Public Address:", key_pair.address)
                # print("Private Key:", key_pair.private_key)
                print(f"Private key: <redacted>")
                yield key_pair

            max_prefix_collisions = num_prefix_collisions
            print(f"{public_address.value} ({max_prefix_collisions}/{prefix_length})")

            # If the current public address matches the desired prefix exactly, print the private key and public address and exit
            if num_prefix_collisions == prefix_length:
                break


# call via:
# python searcher.py --prefix 0x000000 --password execution/keystore/password.txt --output_dir execution/keystore
def main() -> None:
    import argparse

    parser = argparse.ArgumentParser(
        description="Generate an Ethereum public address with a given prefix"
    )
    parser.add_argument(
        "--prefix", type=str, help="Desired prefix for the public address"
    )
    parser.add_argument(
        "--password", type=str, help="Password file to encrypt the private key"
    )
    parser.add_argument(
        "-o", "--output_dir", type=str, help="Output directory for the keystore file"
    )
    args = parser.parse_args()

    prefix: str = args.prefix
    out: str = args.output_dir

    # read password from execution/keystore/password.txt
    pw = args.password

    for keypair in search_for_prefix(prefix):
        encrypt_key_file(keypair.private_key, pw, output_dir=out)


if __name__ == "__main__":
    main()
