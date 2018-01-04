#!/usr/bin/env python

# Python2

import sys
import json
import struct
from subprocess import Popen, STDOUT
import os
from time import sleep


def oneOf(one, two, three):
    if os.path.exists(one): return one
    if os.path.exists(two): return two
    if os.path.exists(three): return three

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

cmd = oneOf("/Applications/SeaMonkey.app/Contents/MacOS/seamonkey",
                      "/usr/local/seamonkey/seamonkey",
                      "C:\\Program Files\\mozilla.org\\SeaMonkey\\seamonkey.exe")

while True:
    receivedMessage = getMessage()
    if receivedMessage.startswith("edit: "):
        launch([cmd, "-edit", receivedMessage.split(": ")[1]])
        sendMessage(encodeMessage("ok"))
    elif receivedMessage == "quit":
        sendMessage(encodeMessage("ok"))
        sleep(5)
        exit(0)
    else:
        sendMessage(encodeMessage("Command not understood"))
