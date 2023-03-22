from dataclasses import dataclass
from typing import Any, Dict

from Crypto.Protocol.KDF import scrypt  # type: ignore

##############################################################################
# Key Derivation Function (KDF) Parameters
##############################################################################


@dataclass(frozen=True)
class KDFParamsPartial:
    """Partial parameters for the key derivation function."""

    dklen: int
    n: int
    r: int
    p: int


@dataclass(frozen=True)
class KDFParams:
    """Parameters for the key derivation function."""

    dklen: int  # length of the derived key in bytes
    salt: bytes  # salt used for the KDF function
    N: int  # number of iterations of the KDF function
    p: int  # the number of iterations of the inner hash function
    r: int  # the memory size in kibibytes


@dataclass(frozen=True)
class KDF:
    """Key derivation function."""

    kdf: str
    params: KDFParams
    derived_key: bytes

    def to_dict(self) -> Dict[str, Any]:
        """Convert the KDF object to a dictionary."""
        return {
            "kdf": self.kdf,
            "kdfparams": {
                "dklen": self.params.dklen,
                "n": self.params.N,
                "p": self.params.p,
                "r": self.params.r,
                "salt": self.params.salt.hex().lower(),
            },
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any], password: str) -> "KDF":
        """Create a KDF object from a dictionary."""
        # Decrypt the private key using the provided password
        salt: bytes = bytes.fromhex(data["kdfparams"]["salt"])
        N: int = data["kdfparams"]["n"]
        r: int = data["kdfparams"]["r"]
        p: int = data["kdfparams"]["p"]
        dklen: int = data["kdfparams"]["dklen"]

        derived_key = scrypt(
            password.encode(),
            salt=salt,
            key_len=dklen,
            N=N,
            r=r,
            p=p,
        )
        return cls(
            kdf=data["kdf"],
            params=KDFParams(
                dklen=dklen,
                salt=salt,
                N=N,
                r=r,
                p=p,
            ),
            derived_key=derived_key,
        )


def scrypt_kdf(password: str, salt: bytes) -> KDF:
    N: int = 4096
    r: int = 8
    p: int = 6
    dklen: int = 32

    # Derive a key from the password and salt using the scrypt key derivation function
    derived_key = scrypt(
        password.encode(),
        salt=salt,
        key_len=dklen,
        N=N,
        r=r,
        p=p,
    )

    print(f"encrypt salt:           {salt.hex()}")
    print(f"encrypt derived_key:    {derived_key.hex()}")

    return KDF(
        kdf="scrypt",
        params=KDFParams(dklen=dklen, salt=salt, N=N, r=r, p=p),
        derived_key=derived_key,
    )
