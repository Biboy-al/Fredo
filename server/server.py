from flask import Flask
import random
import json
import fileOps

app = Flask(__name__)

random.seed(10)
@app.route("/clients")
def all_clients():
    return "These are clients"

@app.route("/logs")
def get_log():
    return "this is a log"

@app.route("/Postlogs", methods=['POST'])
def connect():
    return "this is a log"

@app.route("/registry")
def registry():

    malwareRegistry = str(random.random())

    malware = {
        "id": malwareRegistry
    }
    
    entry = json.dumps(malware)

    fileOps.add_malware(entry)

    return malwareRegistry
    


if __name__ == "__main__":
    app.run(debug=True)