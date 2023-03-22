import json
import os
import uuid
from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Any, Dict

from core.address import Address, PrivateKey
from core.cipher import Cipher, CipherParams
from core.encrypt import Crypto
from core.kdf import KDF, scrypt_kdf
from Crypto.Cipher import AES  # type: ignore
from Crypto.Hash import keccak  # type: ignore
from Crypto.Util import Counter  # type: ignore


@dataclass(frozen=True)
class Keystore:
    """Keystore file containing the encrypted private key."""

    address: str
    crypto: Crypto
    id: uuid.UUID
    version: int

    def json(self) -> Dict[str, Any]:
        """Return the JSON representation of the keystore object."""
        return {
            "address": self.address[2:].lower(),
            "crypto": {
                **self.crypto.to_dict(),
            },
            "id": str(self.id),
            "version": 3,
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any], password: str) -> "Keystore":
        """Create a Keystore object from a dictionary."""
        return Keystore(
            address=data["address"],
            crypto=Crypto.from_dict(data["crypto"], password),
            id=uuid.UUID(data["id"]),
            version=data["version"],
        )

    def dump(self) -> None:
        print(f"Keyfile:")
        print(f"address: {self.address}")
        print(f"crypto:")
        print(f"  cipher: {self.crypto.cipher.cipher}")
        print(f"  ciphertext: {self.crypto.cipher.ciphertext.hex()}")
        print(f"  cipherparams:")
        print(f"    iv: {self.crypto.cipher.params.iv.hex()}")
        print(f"  kdf: {self.crypto.kdf.kdf}")
        print(f"    dklen: {self.crypto.kdf.params.dklen}")
        print(f"    n: {self.crypto.kdf.params.N}")
        print(f"    p: {self.crypto.kdf.params.p}")
        print(f"    r: {self.crypto.kdf.params.r}")
        print(f"    salt: {self.crypto.kdf.params.salt.hex()}")
        print(f"id: {self.id}")
        print(f"version: {self.version}")


@dataclass(frozen=True)
class KeystoreFile:
    """Keystore file containing the encrypted private key."""

    filepath: str
    keystore: Keystore


def keystore_filename(address: str) -> str:
    """Generate a keystore filename for an Ethereum address."""
    if address.startswith("0x"):
        address = address[2:]
    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H-%M-%S.%f")[:-3] + "Z"
    return f"UTC--{timestamp}--{address}"


##############################################################################
# Helper Functions
##############################################################################


#
def encrypt_key(private_key: PrivateKey, password: str) -> Keystore:
    """Generate a keystore file for an Ethereum private key."""

    # Generate a random 32-byte salt
    salt = os.urandom(32)

    # Derive a key from the password and salt using scrypt
    kdf: KDF = scrypt_kdf(password, salt)

    # Use the derived key to encrypt the private key
    aes_key: bytes = kdf.derived_key[:16]
    iv: bytes = os.urandom(16)
    aes = AES.new(
        aes_key,
        AES.MODE_CTR,
        counter=Counter.new(128, initial_value=int.from_bytes(iv, byteorder="big")),
    )
    assert (
        len(private_key.value) % AES.block_size == 0
    ), "Geth implementation does not pad, so PK must align with block size"
    ciphertext = aes.encrypt(private_key.value)
    cipher = Cipher(
        cipher="aes-128-ctr", ciphertext=ciphertext, params=CipherParams(iv=iv)
    )

    # Generate a MAC using Keccak-256 hash
    mac = keccak.new(digest_bits=256)
    mac.update(kdf.derived_key[16:32] + ciphertext)
    mac_digest = mac.digest()

    crypto: Crypto = Crypto(
        cipher=cipher,
        kdf=kdf,
        mac=mac_digest,
    )

    address: Address = Address.from_pk(private_key)
    print(f"Address: {address}")
    # print(f"Private key: {private_key.value.hex()}")
    print(f"Private key: <encrypted>")
    id_ = uuid.uuid4()

    keystore: Keystore = Keystore(
        address=address.value,
        crypto=crypto,
        id=id_,
        version=3,
    )

    return keystore


def encrypt_key_file(
    private_key: PrivateKey, password: str, output_dir: str
) -> KeystoreFile:
    """Generate a keystore file for an Ethereum private key."""
    keystore: Keystore = encrypt_key(private_key=private_key, password=password)

    # Generate the file name using the UTC timestamp and Ethereum address
    filename: str = keystore_filename(address=keystore.address)
    filepath: str = os.path.join(output_dir, filename)
    relpath = os.path.relpath(filepath, os.path.dirname(os.path.realpath(__file__)))
    print(f"Saving keystore file to {relpath}")
    with open(filepath, "w") as f:
        # no spaces nor newlines in the JSON output
        json.dump(keystore.json(), f, separators=(",", ":"))
        printed = json.dumps(keystore.json(), indent=4)
        print(f"Keystore file saved to {relpath}\n{printed}")

    return KeystoreFile(filepath=filepath, keystore=keystore)
