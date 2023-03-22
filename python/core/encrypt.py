import os
from dataclasses import dataclass
from typing import Any, Dict

from Crypto.Cipher import AES  # type: ignore
from Crypto.Util.Padding import pad  # type: ignore

from .address import PrivateKey
from .cipher import Cipher, CipherParams
from .kdf import KDF


@dataclass(frozen=True)
class Crypto:
    """Crypto parameters."""

    kdf: KDF
    cipher: Cipher
    mac: bytes

    def to_dict(self) -> Dict[str, Any]:
        """Convert the Crypto object to a dictionary."""
        return {
            **self.cipher.to_dict(),
            **self.kdf.to_dict(),
            "mac": self.mac.hex().lower(),
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any], password: str) -> "Crypto":
        """Create a Crypto object from a dictionary."""
        return cls(
            cipher=Cipher.from_dict(data),
            kdf=KDF.from_dict(data, password),
            mac=bytes.fromhex(data["mac"]),
        )


def aes128ctr(private_key: PrivateKey, kdf: KDF) -> Crypto:
    """Encrypt the private key using AES-128-CTR."""
    assert isinstance(private_key, PrivateKey)
    # Generate a new IV (initialization vector) for the AES-128-CTR cipher
    iv = os.urandom(AES.block_size)
    # Encrypt the private key using AES-128-CTR
    aes_key: bytes = kdf.derived_key[:16]
    aes = AES.new(aes_key, AES.MODE_CTR, nonce=iv)
    ciphertext = aes.encrypt(pad(private_key.value, AES.block_size))
    cipherparams: CipherParams = CipherParams(iv=iv)

    cipher_result = Cipher(
        cipher="aes-128-ctr",
        ciphertext=ciphertext,
        params=cipherparams,
    )

    return Crypto(cipher=cipher_result, kdf=kdf)
