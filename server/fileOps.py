import json
import os

def add_malware(malware_entry):
    entry = json.dumps(malware_entry)
    file_name = f"malware/id_{malware_entry["id"]}.txt"

    os.makedirs("malware", exist_ok="true")

    with open(file_name, "w") as f:
        f.write(entry + "\n")

def update_becon_malware(id, timestamp):

    malware_json = json.loads(get_malware(id))

    malware_json["last_beconed"] = timestamp

    add_malware(malware_json)


def get_malware(id):

    file_name = f"malware/id_{id}.txt"

    os.makedirs("malware", exist_ok="true")

    with open(file_name, "r") as f:
        return f.read()
    