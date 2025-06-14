
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
    ciphertext = base64.b64decode(ciphertext)
    plaintext = private_key.decrypt(
    ciphertext,
    padding.PKCS1v15()
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

cipher_text = "T5mEw0+d6SFQnjVcnaBBkz42NIUMP3p6k0j3E1VR2jOp3hU//gImJQxIAgyNtPDANXrYORlA5jm0iyww78cw7EdMJM6czpTwunJ3CoXllC3JV/6FZwO2dXmd/9lspAQAEJu53grVdL4oVwtS79CTPcVpq1KSTFE24YGp1mLgyqAoVhW76B5Z2Lkh/PDQtW99xfw79r491BnNST3tcqQvLaWDV0VOTJC684PcdydzlVMjXb3JHUvSQ+yJYsdzokGGaxUBFlhcgWpB0dhDPXug2djEqsNOwLVzgygnWELE1IxTAYmNtsrail9GCveDz6oAgU9j8vBH5P+qXpi78Kpjjg=="

decrypt_message_priv(cipher_text)