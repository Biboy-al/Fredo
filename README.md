# Fredo

Fredo, to me is one of the most gut renching betrayals in cnimatic histoy. I am of course talking about GodFather 2. This malware, just like Fredo, will infect your system loosgining your guard, and when you least expect it will betray you (although it will betray you from the start).

Call me theatrical, but I like calling my projects names, it creates a deeper link, and invests me in it's development.

## What is this?

Fredo, is trojan writen in Rust. It will act as both a keyloggering, and remote code execition from a C2 server.

## Architecture

The malware will consits of two main components:

- C2 Server: This is where the malware will registry itself, exfiltrate data, and run remote commands.
- Maleware : This is the malware it self, logging key strokes.

## Capabilities

### Server

|Function| done?|
|--------|------|
| Log connected c2 server | |
| generate unique ID based on host | |
| send beacon signals to malware | |

#### Defensive Capabilities:


- Anti-sanbox
  1. Upon inital execution the malware will sleep for 10 minutes. Sandboxes have a predefined timer, and this is function will wait out it's timer. 
  2. Malware takes a snapshot of running process. If process is < 30, malware will exit. Sandboxes run on minimal number of process.
- Anti Static Analysis
  1. The final binary of the malware will be packed
  2. Malware has dead branches, i.e if statements that will always be false or true. This aims to confuse static analysis programs.
- Anti Debugger
  1. Malware uses the "IsDebuggerPresent" windows function. Checking whenever the malware is attached to a debugger if it is exit.
- Anti VM
  1. Malware takes a snapshot of the running process, and checks "VMbox.exe" etc. if see exit.
- Discrete keylogging file
  1. To make it more legitimite, the file used to store and exfiltrate the data, will be stored in a secure place. calling itself the "security.log" file.
- Discrete Malware
  1. Upon inital execution, malware will move itself to a legitimate position "System32/micrisoftSystemUpdater.exe".
- Network signature:
  1. The requests sent to the C2 server will be sent at random intervals. This makes it harder for an analysit to check for behaviour.
- Unspoofable C2 Server
  1. Malware and C2 uses public key encrytpion to establish shared key. This prevent the C2 server from being spoofed and send data to the malware.
- Persistency
  1. Malware adds itself as a schedulled task. When user exit, and opens windows the malware will run again.
 
  


### Malware

|Function| done?|
|--------|------|
| Keylogging  | |
| Exfiltrate data | |
| Persistency | |
| Remote code execution | |


## Limitations

|Limit| Description|
|--------|------|
| Lack of forward security | |
| Exfiltrate data | |
| Persistency | |
| Remote code execution | |

## How to Build the project


### Malware

Requirments: To run the malware, you will need to have installed rust on your system.

#### To run and test the code:

1. Cd into the "Fredo" directory
2. Then run this command:
```
cargo run
```
Note: This command will also download any dependenices 

#### To compile the rust program

To compile the malware into a executable for a 32-bit windows system:
```
rustup target add i686-pc-windows-gnu
```

```
cargo build --target=i686-pc-windows-gnu
```

To compile the malware into an executable for a 64-bit windows system:

```
cargo build
```


### Server

To run the server:
1. cd into server folder
2. install all dependencies
3. Run this command
```
Python3 server.py
```

