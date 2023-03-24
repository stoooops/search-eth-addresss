from hashlib import sha3_256
from typing import Union

import ecdsa
from Crypto.Hash import keccak  # type: ignore
from ecdsa import SECP256k1, SigningKey
from hdwallet import BIP32HDWallet, HDWallet
from mnemonic import Mnemonic


def eth_pubkey_to_address(pubkey):
    keccak_hash = sha3_256()
    keccak_hash.update(pubkey[1:])
    return "0x" + keccak_hash.hexdigest()[24:]


phrase = "dust royal enter exhaust hand hood fork tree flush goat iron rookie job power gold remember small luxury raw broccoli access helmet left fame"
print(f"phrase       : {phrase}")
seed: bytes = Mnemonic.to_seed(phrase, passphrase="")
print(f"seed         : 0x{seed.hex()}")

# # Create a BIP39 mnemonic and binary seed
# # entropy = os.urandom(32)
# # mnemo = Mnemonic("english")
# # mnemonic = mnemo.to_mnemonic(entropy)
# # seed = mnemo.to_seed(mnemonic, passphrase="")

# # Generate the BIP32 master key and derive the Ethereum "m/44'/60'/0'/0/0" child key
# hdwallet: HDWallet = BIP32HDWallet(symbol="ETH").from_seed(seed.hex())
# # hdwallet.from_entropy(entropy="ee535b143b0d9d1f87546f9df0d06b1a")
# hdwallet.from_path(path="m/44'/60'/0'/0")
# # print("xpub", hdwallet.xpublic_key())
# # print("uncompressed :", hdwallet.uncompressed())
# # print("compressed   :", hdwallet.compressed())
# print(f"private key  : 0x{hdwallet.private_key()}")
# print(f"public  key  : 0x{hdwallet.public_key(compressed=True)}")
# print(f"public  key  : 0x{hdwallet.public_key(compressed=False)}")
# print(f"address      : 0x{hdwallet.address()}")


# # bip32_root = BIP32HDWallet("ETH").from_seed(seed)
# # eth_child_key = bip32_root.from_path("m/44'/60'/0'/0/0")

# # # Get the Ethereum address
# # signing_key = SigningKey.from_string(eth_child_key.private_key(), curve=SECP256k1)
# # eth_address = eth_pubkey_to_address(signing_key.get_verifying_key().to_string())
# # print(f"Ethereum Address: {eth_address}")


# def decompress_public_key(compressed_key: Union[str, bytes]) -> bytes:
#     if isinstance(compressed_key, str):
#         if compressed_key.startswith("0x"):
#             compressed_key = compressed_key[2:]
#         compressed_key = bytes.fromhex(compressed_key)

#     curve = ecdsa.SECP256k1
#     prefix = compressed_key[0]
#     if prefix not in (2, 3):
#         raise ValueError(f"Invalid compressed public key prefix: {prefix}")

#     x = int.from_bytes(compressed_key[1:], byteorder="big")
#     p = curve.curve.p()
#     y_squared = (pow(x, 3, p) + curve.curve.a() * x + curve.curve.b()) % p
#     y = pow(y_squared, (p + 1) // 4, p)

#     if (y % 2) != (prefix % 2):
#         y = p - y

#     uncompressed_key = (
#         b"\x04" + x.to_bytes(32, byteorder="big") + y.to_bytes(32, byteorder="big")
#     )
#     print(f"uncompressed : 0x{uncompressed_key.hex()}")
#     return uncompressed_key


# decompress_public_key(hdwallet.public_key(compressed=True))
# addr = eth_pubkey_to_address(
#     decompress_public_key(hdwallet.public_key(compressed=True))
# )
# print(f"address      : 0x{addr}")
# addr = eth_pubkey_to_address(
#     decompress_public_key(
#         "02e48450d9f1fe6cd8b422e23c075153e2aba775224fe5b614a8e960e19574a4bc"
#     )
# )
# print(f"address      : 0x{addr}")


# print("-------------")
# print(len(seed))
hdwallet = BIP32HDWallet(symbol="ETH").from_private_key(
    "b905f3fcfb674b1f1fecb064672a8ec0b449fa1e289172b70ce8b39d010ef926"
)

# hdwallet.from_path(path="m/44'/60'/0'/0")
# print("xpub", hdwallet.xpublic_key())
# print("uncompressed :", hdwallet.uncompressed())
# print("compressed   :", hdwallet.compressed())
print(f"private key  : 0x{hdwallet.private_key()}")
print(f"public  key  : 0x{hdwallet.public_key(compressed=True)}")
# print(f"address      : 0x{hdwallet.address()}")

uncompressed_public_key: str = hdwallet.public_key(compressed=False)
if uncompressed_public_key.startswith("0x"):
    uncompressed_public_key = uncompressed_public_key[2:]
print(f"uncompressed : 0x{uncompressed_public_key}")
uncompressed_public_key_bytes = bytes.fromhex(uncompressed_public_key)

assert len(uncompressed_public_key_bytes) == 64, f"{len(uncompressed_public_key_bytes)}"


keccak_256 = keccak.new(digest_bits=256)
keccak_256.update(uncompressed_public_key_bytes)
hashed = keccak_256.hexdigest()
assert len(hashed) == 64
print(f"keccak256    : 0x{hashed}")

address = keccak_256.hexdigest()[24:]
assert len(address) == 40

print(f"address      : 0x{address}")
