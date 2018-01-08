#!/usr/bin/env python
# Adapted from https://github.com/mdn/webextensions-examples/blob/master/native-messaging/app/ping_pong.py
# Python2

import sys
import json
import struct
from subprocess import Popen, STDOUT
import os
from time import sleep

def oneOf(executables):
    for executable in executables:
        if os.path.exists(executable): return executable
    return None

# Read a message from stdin and decode it.
def getMessage():
    rawLength = sys.stdin.read(4)
    if len(rawLength) == 0:
        sys.exit(0)
    messageLength = struct.unpack('@I', rawLength)[0]
    message = sys.stdin.read(messageLength)
    return json.loads(message)

# Encode a message for transmission,
# given its content.
def encodeMessage(messageContent):
    encodedContent = json.dumps(messageContent)
    encodedLength = struct.pack('@I', len(encodedContent))
    return {'length': encodedLength, 'content': encodedContent}

# Send an encoded message to stdout
def sendMessage(encodedMessage):
    sys.stdout.write(encodedMessage['length'])
    sys.stdout.write(encodedMessage['content'])
    sys.stdout.flush()

def launch(cmd_with_arg):
    with open(os.devnull, 'r+b', 0) as DEVNULL:
        Popen(cmd_with_arg, stdin=DEVNULL, stdout=DEVNULL, stderr=STDOUT, close_fds=True)

if os.name == 'nt':
    executables = ["C:\\Program Files\\mozilla.org\\SeaMonkey\\seamonkey.exe"]
elif os.name == 'mac':
    executables = ["/Applications/SeaMonkey.app/Contents/MacOS/seamonkey"]
else:
    executables = ["/usr/local/seamonkey/seamonkey"]

cmd = oneOf(executables)

# So the event loop is a little weird. If you could not guess
# it is the "WebExtensions" stdio API over STDIN and STDOUT, a message size
# then a message is sent. The latter is JSON, but the contents of the JSON
# need not be.  Firefox and Chrome honor the WebExtensions API.

while True:
    receivedMessage = getMessage()
    if receivedMessage.startswith("edit: "):
        if (cmd is None):
            sendMessage(encodeMessage("Error: None of these executable paths exist: "
                                      + str(executables) + ", therefore no launching of an editor!"))
        else:
            launch([cmd, "-edit", receivedMessage.split(": ")[1]])
            sendMessage(encodeMessage("ok"))
    elif receivedMessage == "quit":
        sendMessage(encodeMessage("ok"))
        sleep(5)
        exit(0)
    else:
        sendMessage(encodeMessage("Command not understood"))
