#!/usr/bin/env python3

"""
Ethereum Keystore Encryption Script

This script generates an Ethereum keystore file for a given private key and saves it to a specified file.
The keystore file is encrypted using AES-128-CTR and scrypt using the provided password. Script output,
along with password file, can be used in geth to unlock an account.

Usage:
    python encrypter.py <private_key> <output_file>

Arguments:
    private_key (str): The private key to generate the keystore file for.
    output_file (str): The file path to save the generated keystore file.

Example:
    python encrypter.py 0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef keystore.json
"""


import argparse
import json

from core.address import PrivateKey
from core.keystore import Keystore, encrypt_key


def main() -> None:
    """Main function."""

    parser = argparse.ArgumentParser(
        description="Generate an Ethereum public address with a given prefix"
    )
    parser.add_argument(
        "pk",
        type=str,
        help="The private key to generate the keystore file for",
    )
    parser.add_argument(
        "outfile",
        default="keystore.json",
        type=str,
        nargs="?",
        help="The file to write the keystore file to",
    )
    args = parser.parse_args()

    pk: PrivateKey = PrivateKey.from_hex(args.pk)
    out: str = args.outfile

    # Generate a keystore file for the private key
    keystore: Keystore = encrypt_key(private_key=pk, password="")
    with open(out, "w") as f:
        # no spaces nor newlines in the JSON output
        json.dump(keystore.json(), f, separators=(",", ":"))
        printed = json.dumps(keystore.json(), indent=4)
        print(f"Keystore file saved to {out}")
        print(f"{printed}")


if __name__ == "__main__":
    main()
