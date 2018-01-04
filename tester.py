#!/usr/bin/env python

# Python 2

import sys
import json
import struct
from subprocess import Popen, PIPE, STDOUT
import os
from time import sleep
from termcolor import colored

cmd_path = os.getcwd() + "/" + "edit_this_page.py"
print("cmd path: " + cmd_path)
edit_this_page = Popen(["python2", cmd_path], stdout=PIPE, stdin=PIPE, stderr=STDOUT)

# Read a message from stdin and decode it.
def getMessage():
    rawLength = edit_this_page.stdout.read(4)
    if len(rawLength) == 0:
        sys.exit(0)
    messageLength = struct.unpack('@I', rawLength)[0]
    message = edit_this_page.stdout.read(messageLength)
    return json.loads(message)

# Encode a message for transmission,
# given its content.
def encodeMessage(messageContent):
    encodedContent = json.dumps(messageContent)
    encodedLength = struct.pack('@I', len(encodedContent))
    return {'length': encodedLength, 'content': encodedContent}

# Send an encoded message to stdout
def sendMessage(encodedMessage):
    edit_this_page.stdin.write(encodedMessage['length'])
    edit_this_page.stdin.write(encodedMessage['content'])
    edit_this_page.stdin.flush()

def color_it(msg):
    if msg == "ok":
        return colored('ok', 'green')
    else:
        return colored(msg, 'red')

print("Asking edit_this_page.py to edit a page ...")
sendMessage(encodeMessage("edit: https://en.wikipedia.org/wiki/Iron_Fist_(TV_series)"))
print("... response from edit_this_page.py: " + color_it(getMessage()))

sleep(5)

print("Asking edit_this_page.py to edit another page ...")
sendMessage(encodeMessage("edit: https://en.wikipedia.org/wiki/Luke_Cage_(TV_series)"))

print("... response from edit_this_page.py: " + color_it(getMessage()))

# ^ Unfortunately SeaMonkey doesn't want to launch a second exe concurrently :-(

print("Asking edit_this_page.py to blahblahblah ...")
sendMessage(encodeMessage("blahblahblah"))
print("... response from edit_this_page.py: " + color_it(getMessage()))

print("Quitting after two page launches and that bad-command test ... ")
sendMessage(encodeMessage("quit"))
reply = getMessage()
edit_this_page.wait()
print("... (" + color_it(reply) + ") done.")
