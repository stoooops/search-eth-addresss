from dataclasses import dataclass
from typing import Any, Dict


@dataclass(frozen=True)
class CipherParams:
    """Cipher parameters."""

    iv: bytes


@dataclass(frozen=True)
class Cipher:
    """Cipher parameters."""

    cipher: str
    ciphertext: bytes
    params: CipherParams

    def to_dict(self) -> Dict[str, Any]:
        """Convert the Cipher object to a dictionary."""
        return {
            "cipher": self.cipher,
            "ciphertext": self.ciphertext.hex().lower(),
            "cipherparams": {"iv": self.params.iv.hex().lower()},
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Cipher":
        """Create a Cipher object from a dictionary."""
        return cls(
            cipher=data["cipher"],
            ciphertext=bytes.fromhex(data["ciphertext"]),
            params=CipherParams(iv=bytes.fromhex(data["cipherparams"]["iv"])),
        )
