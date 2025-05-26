def add_malware(txt):
    with open("malwares.txt", "a") as f:
        f.write(txt + "\n")

def get_malwares():
    with open("malwares.txt", "r") as f:
        return f.read()