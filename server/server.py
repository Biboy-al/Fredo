from flask import Flask

app = Flask(__name__)

@app.route("/clients")
def all_clients():
    return "These are clients"

@app.route("/logs")
def get_log():
    return "this is a log"

@app.route("/Postlogs", methods=['POST'])
def connect():
    return "this is a log"


if __name__ == "__main__":
    app.run(debug=True)