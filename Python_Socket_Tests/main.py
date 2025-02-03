import json
import socket


s = socket.socket()
s.connect(("127.0.0.1",8080))

data = {
    "name":"get_open_windows",
    "args":[]   
}

s.send(json.dumps(data).encode())