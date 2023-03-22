#!/usr/bin/env python3

"""
Ethereum Keystore Decryption Script

This script decrypts an Ethereum keystore file using a provided password and outputs the corresponding private key.
If no password is provided, it prompts the user to input one.

Usage:
    python decrypter.py <input_file> [--password <password>]

Arguments:
    input_file (str): The file path of the keystore file to decrypt.
    password (str, optional): The password to decrypt the keystore file with.

Example:
    python eth_keystore_decryptor.py keystore.json --password my_password
"""


import argparse
import json
from getpass import getpass
from typing import Any, Dict

from core.address import PrivateKey
from core.keystore import Keystore
from core.unlock_account import unlock_account


def read(keyfile_path) -> Dict[str, Any]:
    with open(keyfile_path, "r") as keyfile:
        return json.load(keyfile)


def main() -> None:
    """Main function."""

    parser = argparse.ArgumentParser(
        description="Generate an Ethereum public address with a given prefix"
    )
    parser.add_argument(
        "infile",
        type=str,
        help="The private key to generate the keystore file for",
    )
    parser.add_argument(
        "--password",
        type=str,
        help="The password to decrypt the keystore file with",
        required=False,
    )
    args = parser.parse_args()

    infile: str = args.infile
    contents: Dict[str, Any] = {}
    with open(infile, "r") as f:
        contents = json.loads(f.read())

    password = (
        args.password
        if args.password is not None
        else getpass(prompt="Enter password to decrypt keyfile: ")
    )

    keystore: Keystore = Keystore.from_dict(contents, password)
    keystore.dump()
    private_key: PrivateKey = unlock_account(keystore, password)
    print(private_key.hex())


if __name__ == "__main__":
    main()
