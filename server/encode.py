
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import padding
import base64
import hashlib
import random


with open("private_key.pem", "rb") as key_file:
    private_key = serialization.load_pem_private_key(
        key_file.read(),
        password=None,
    )


with open("public_key.pem", "rb") as key_file:
    public_key = serialization.load_pem_public_key(
        key_file.read()
    )

def decrypt_message_priv(ciphertext):
    ciphertext = base64.b64decode(ciphertext)
    plaintext = private_key.decrypt(
    ciphertext,
    padding.PKCS1v15()
    )

    return plaintext.decode()
    
def xor_encrypt(plaintext, key):
    plaintext_bytes = plaintext.encode('utf-8')
    encrypted_bytes = bytearray()

    for byte in plaintext_bytes:
        encrypted_bytes.append(byte ^ key)

    # Encode the XORed bytes as base64 string
    return base64.b64encode(encrypted_bytes).decode('utf-8')


def xor_decrypt(base64_ciphertext, key):
    # Decode base64 string to bytes
    ciphertext_bytes = base64.b64decode(base64_ciphertext)
    decrypted_bytes = bytearray()

    for byte in ciphertext_bytes:
        decrypted_bytes.append(byte ^ key)

    return decrypted_bytes.decode('utf-8')


def generate_id(s, min_val=100000, max_val=900000):
    seed = int(hashlib.md5(s.encode()).hexdigest(), 16)
    rng = random.Random(seed)
    return rng.randint(min_val, max_val)