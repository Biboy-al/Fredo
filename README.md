# CYBR473 MALWARE

## Architecture

The malware will consits of two main components:

- C2 Server: This is where the malware will registry itself, exfiltrate data, and run remote commands.
- Maleware : This is the malware it self, logging key strokes.
- Attacker : This is a way to communicate with the malware. Allowing to view registered malware, logs, and send commands.


## How to Build the project / Test

### Server

Start off by starting the server:
1. cd into server folder
2. install all dependencies (Flask, Cryptogrpahy, using pip install <lib>)
3. Run this command
```
Python3 server.py
```

### Malware

Requirments: To run the malware, you will need to have installed rust on your system.

#### To run and test the code:

1. Cd into the "Fredo" directory
2. install all dependencies (requests, using pip install <lib>)
3. Then run this command:
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

as a tool to interact with the server, a attacker cli tool is made for you. to run it:

1. cd into the attacker directory
2. python attacker.py <commands>

All commands:

lists - Lists all registered malware
```
python attacker.py list
```
log - obtains the key logs of a certain malware
```
python attacker.py log <id>
```
pwn - sends a string to display on the malware
```
python attacker.py pwn "<msg>" <id>
```
slp - commands the malware to sleep for an amount of time
```
python attacker.py log slp <sec> <id>
```
shd - tells a certain malware to shut down
```
python attacker.py log  shd <id>
```


## Server

### Dependenices 
- flask
- Cryptography

### Endpoints
- **GET /clients**  
  Returns a list of all registered malware instances.

- **GET /logs?id={id}**  
  Retrieves the keystroke logs collected from a specific client identified by the given ID.

- **POST /upload**  
  Receives encrypted log data from a client, decrypts it, and saves it to the system.

- **POST /register**  
  Registers a new malware instance by decrypting the incoming data and storing its information.

- **POST /becon**  
  Updates the last seen timestamp for a malware instance during its periodic beaconing.

- **GET /command**  
  Retrieves the latest command for a malware instance, encrypting the response using its assigned key.

- **POST /command**  
  Posts a new command to be executed by a specific malware instance.


## Malware

### Defensive Capabilities:

- The defender may use antivirus software to detect the malware.
  1. The final binary will be packed. This alters the structure of the malware, evading signature based detection.

- The defender will collect packet traces from the network and use traffic analysis to identify infected hosts (e.g., via their data exfiltration and beaconing), and develop network-based signatures to block its traffic at the network gateway.
  1. The requests sent to the C2 server will be sent at random intervals. This makes it harder for the malware to infer a trends between the malware and c2. Potentially hiding it with the traffic.

- The defender may look for evidence of keylog files on infected machines.
  1. To make it more legitimite, the file used to store and exfiltrate the data, will be stored in a secure place. calling itself the "security.log" file.

- Reboot the client machines so that the malware will be forced to shut down.
  1.  Malware adds itself as a schedulled task. When user exit, and opens windows the malware will run again. This sets up persistency.

- If the defender identifies the infected machine and obtains a copy of your malware, they may first send it to a service like VirusTotal for signature matching or basic static analysis.
  1. The final binary of the malware will be packed.
  2. Malware has dead branches, i.e if statements that will always be false or true. This aims to confuse static analysis programs.
 
- They are likely to use a disassembler like IDA Pro or Ghidra to analyze your malware.
  1. The malware uses macros, to insert dead branches throughout the code. This causes more confusing code with the disassembler.

- The defender may run your malware inside a standard sandbox for quick dynamic analysis.
  1. Upon inital execution the malware will sleep for 10 minutes. Sandboxes have a predefined timer, and this is function will wait out it's timer.
  2. Malware takes a snapshot of running process. If process is < 30, malware will exit. Sandboxes run on minimal number of process.

- They may also use a debugger like OllyDbg or x64dbg for deep dynamic analysis—setting breakpoints, tracing execution, or stepping through the code.
  1. Malware uses the "IsDebuggerPresent" windows function. Checking whenever the malware is attached to a debugger if it is exit.
  
- The defender may analyze your malware in a virtual machine environment (e.g., using VirtualBox).
 1. Malware takes a snapshot of the running process, and checks "VMbox.exe" etc. if see exit.
    
- The malware analyst will eventually find the hard-coded key and decipher the malware's communications with the C2.
  1. The Xor Key used to encrypt the data is randomized during execution. This makes it harder for analyst to detrmine the key, needing to look through the memory.

- The analyst might spoof the C&C server to send a shutdown command to all infected hosts on the network (a "kill-switch").
  1. Upon first regestration, the malware generates a randomized key and encrypts it using the the public key of the c2 server. The c2 server then decrypts using the private key, and encrypts further messages using symettric key. The key establishment ensures that only c2 server can obtain the generated key, ensuring that it is the legitmate server.

- The defender may look for suspicious processes or unusual binaries on disk and trust only processes signed by Microsoft or located in C:\Windows\System32.
  1. Upon inital execution, malware will move itself to a legitimate directory "System32" and call itself "micrisoftSystemUpdater.exe".


## Limitations

|Limit| Description|
|--------|------|
| Lack of forward security | The malware will generate, establish only one xor key throughout it's life time. This means that if the key we're somehow to be exposed, it will compromise the confidentaility of the communication |
| Weak key | For both network, and file encryption, it uses a xor encrpytion. this means that the key has a possible value of 1 - 255. This makes it grealty insecure, as a analyst can easily brute force it until it finds the plain text |
|Limited id deriving function| The c2 server derives the id of the malware from it's OS fingerprint. This means that hosts with the same fingerprint results with the same id.|
