
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives import hashes
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
    
def encrypt_message_priv(plaintext):

    encrypted = public_key.encrypt(
    plaintext.encode(),
    padding.OAEP(
        mgf=padding.MGF1(algorithm=hashes.SHA256()),
        algorithm=hashes.SHA256(),
        label=None
        )
    )

    return base64.b64encode(encrypted).decode()

enc = encrypt_message_priv("Hi mr what is your name?")

dec = decrypt_message_priv(enc)

