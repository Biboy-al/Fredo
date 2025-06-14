
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives import hashes
from cryptography.fernet import Fernet
import base64


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
    plaintext = private_key.decrypt(
    base64.b64decode(ciphertext),
    padding.OAEP(
        mgf=padding.MGF1(algorithm=hashes.SHA256()),
        algorithm=hashes.SHA256(),
        label=None
        )
    )

    
    return plaintext.decode()
    
# def encrypt_message_priv(plaintext):

#     encrypted = public_key.encrypt(
#     plaintext.encode(),
#     padding.OAEP(
#         mgf=padding.MGF1(algorithm=hashes.SHA256()),
#         algorithm=hashes.SHA256(),
#         label=None
#         )
#     )

#     return base64.b64encode(encrypted).decode()


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
