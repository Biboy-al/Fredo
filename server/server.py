from flask import Flask, request
import uuid
import random
import json
import fileOps

app = Flask(__name__)

random.seed(10)
@app.route("/clients")
def all_clients():
    return fileOps.get_malwares()

@app.route("/logs")
def get_log():
    return "this is a log"

@app.route("/Postlogs", methods=['POST'])
def connect():
    return "this is a log"

@app.route("/registry")
def registry():

    malwareRegistry = str(uuid.uuid1().int)

    malware = {
        "id": malwareRegistry,
        "ip": request.remote_addr,
        "signature" : ""
    }
    
    entry = json.dumps(malware)

    fileOps.add_malware(entry)

    return malwareRegistry
    


if __name__ == "__main__":
    app.run(debug=True)