import json
import os

def add_malware(malware_entry):
    entry = json.dumps(malware_entry)
    file_name = f"malware/id_{malware_entry["id"]}.txt"

    os.makedirs("malware", exist_ok="true")

    with open(file_name, "w") as f:
        f.write(entry + "\n")

def get_malwares():
    with open("malwares.txt", "r") as f:
        return f.read()