from hashlib import pbkdf2_hmac

from core.keystore import Keystore  # type: ignore
from Crypto.Cipher import AES  # type: ignore


def decrypt(keystore: Keystore, password: str) -> str:
    if keystore.crypto.cipher.cipher != "aes-128-ctr":
        raise ValueError(
            f"Unsupported cipher {keystore.crypto.cipher.cipher}. Only aes-128-ctr is supported"
        )

    if keystore.crypto.kdf.kdf != "scrypt":
        raise ValueError(
            f"Unsupported kdf {keystore.crypto.kdf.kdf}. Only scrypt is supported"
        )

    salt: bytes = keystore.crypto.kdf.params.salt
    n: int = keystore.crypto.kdf.params.N
    r: int = keystore.crypto.kdf.params.r
    p: int = keystore.crypto.kdf.params.p
    dklen: int = keystore.crypto.kdf.params.dklen
    encryption_key = pbkdf2_hmac(
        "sha256",
        password.encode(),
        salt,
        keystore.crypto.kdf.params.N,
        dklen=keystore.crypto.kdf.params.dklen,
    )
    print(f"encryption_key ({len(encryption_key)}B): {encryption_key.hex()}")
    #  derived_key = scrypt(password.encode(), salt, n, r, p, dklen)
    # print(f"derived_key ({len(derived_key)}B): {derived_key.hex()}")

    cipher = AES.new(
        encryption_key, AES.MODE_CTR, nonce=keystore.crypto.cipher.params.iv
    )
    private_key = cipher.decrypt(keystore.crypto.cipher.ciphertext)
    print(f"private_key ({len(private_key)}B): {private_key.hex()}")

    return private_key.hex()
