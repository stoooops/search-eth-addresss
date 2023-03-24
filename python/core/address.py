from dataclasses import dataclass

from Crypto.Hash import keccak  # type: ignore
from ecdsa import SECP256k1, SigningKey, VerifyingKey  # type: ignore


@dataclass(frozen=True)
class PrivateKey:
    """A private key."""

    value: bytes

    def hex(self) -> str:
        return f"{self.value.hex()}"

    @classmethod
    def from_hex(cls, hex: str) -> "PrivateKey":
        """Convert a private key in hex string format to bytes."""
        if not isinstance(hex, str):
            raise TypeError("hex must be a string")

        if hex.startswith("0x"):
            hex = hex[2:]

        if len(hex) != 64:
            raise ValueError(
                f"hex must be 64 characters long (excluding '0x' prefix). Got: {len(hex)}"
            )

        return cls(bytes.fromhex(hex))

    def __str__(self) -> str:
        return self.hex()


@dataclass(frozen=True)
class Address:
    """A public address."""

    value: str

    @property
    def bytes(self) -> bytes:
        return bytes.fromhex(self.value[2:])

    @classmethod
    def from_signing_key(cls, signing_key: SigningKey) -> "Address":
        """Convert an ECDSA signing key to an Ethereum address."""
        # Get the verifying key from the signing key.
        verifying_key: VerifyingKey = signing_key.get_verifying_key()
        # Convert the uncompressed public key to bytes.
        public_key: bytes = verifying_key.to_string("uncompressed")
        # Compute the Keccak-256 hash of the public key bytes.
        # Take the last 20 bytes of the hash.
        # trim the uncompressed public key prefix (0x04) via [1:]
        public_key_hash: bytes = keccak.new(
            digest_bits=256, data=public_key[1:]
        ).digest()[-20:]
        # Prepend the string with "0x" to create the Ethereum address.
        address = "0x" + public_key_hash.hex()
        return Address(address)

    @classmethod
    def from_pk(cls, pk: PrivateKey) -> "Address":
        """Convert a private key to an Ethereum address."""
        assert isinstance(pk, PrivateKey)
        # Convert the 32-byte private key to an ECDSA signing key
        signing_key: SigningKey = SigningKey.from_string(pk.value, curve=SECP256k1)
        # Convert the signing key to an Ethereum address
        return Address.from_signing_key(signing_key)

    def __str__(self) -> str:
        return self.value


@dataclass(frozen=True)
class KeyPair:
    """A public key."""

    address: Address
    private_key: PrivateKey


if __name__ == "__main__":
    public_key_hex = (
        "0x02e48450d9f1fe6cd8b422e23c075153e2aba775224fe5b614a8e960e19574a4bc"
    )
    public_key: bytes = bytes.fromhex(public_key_hex[2:])
    hashed = keccak.new(digest_bits=256, data=public_key[1:]).digest()
    print(hashed.hex())
    # Create a private key
    public_key_hash: bytes = hashed[-20:]
    # Prepend the string with "0x" to create the Ethereum address.
    address = "0x" + public_key_hash.hex()
    print(address)
