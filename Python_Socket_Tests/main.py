import json
import socket


s = socket.socket()
s.connect(("127.0.0.1",8080))

data = {
    "name":"type",
    "args":["20","20"]   
}

s.send(json.dumps(data).encode())