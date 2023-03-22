from core.address import PrivateKey
from core.keystore import Keystore
from Crypto.Cipher import AES  # type: ignore
from Crypto.Hash import keccak  # type: ignore
from Crypto.Protocol.KDF import scrypt  # type: ignore
from Crypto.Util import Counter  # type: ignore


def unlock_account(keystore: Keystore, password: str) -> PrivateKey:
    # Decrypt the private key using the provided password
    salt: bytes = keystore.crypto.kdf.params.salt
    N: int = keystore.crypto.kdf.params.N
    r: int = keystore.crypto.kdf.params.r
    p: int = keystore.crypto.kdf.params.p
    dklen: int = keystore.crypto.kdf.params.dklen

    derived_key = scrypt(
        password.encode(),
        salt=salt,
        key_len=dklen,
        N=N,
        r=r,
        p=p,
    )

    ciphertext = keystore.crypto.cipher.ciphertext

    # Generate a MAC using Keccak-256 hash
    keccak_hash = keccak.new(digest_bits=256)
    keccak_hash.update(derived_key[16:32] + ciphertext)
    mac = keccak_hash.digest()
    print(f"decrypt salt:           {salt.hex()}")
    print(f"decrypt derived_key: {derived_key.hex()}")
    print(f"decrypt mac:            {mac.hex()}")

    # Verify the MAC
    expected_mac = keystore.crypto.mac
    if mac != expected_mac:
        print(f"Expected MAC: {expected_mac.hex()}")
        print(f"MAC:          {mac.hex()}")
        raise ValueError("Incorrect password")

    # Decrypt the private key
    iv = keystore.crypto.cipher.params.iv
    encrypted_private_key = keystore.crypto.cipher.ciphertext
    aes_key: bytes = derived_key[:16]
    aes = AES.new(
        aes_key,
        AES.MODE_CTR,
        counter=Counter.new(128, initial_value=int.from_bytes(iv, byteorder="big")),
    )
    private_key = aes.decrypt(encrypted_private_key)

    return PrivateKey(private_key)
