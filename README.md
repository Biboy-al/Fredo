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

### Malware

|Function| done?|
|--------|------|
| Keylogging  | |
| Exfiltrate data | |
| Persistency | |
| Remote code execution | |
