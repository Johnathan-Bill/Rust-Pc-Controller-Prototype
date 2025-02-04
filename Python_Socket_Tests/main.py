import json
import socket


s = socket.socket()
s.connect(("127.0.0.1",8080))

data = {
    "name":"type",
    "args":["value" * 500 for _ in range(34)]  
}

json_data = json.dumps(data)

# Ensure that the size is around 17 KB (this depends on the JSON structure)
print(f"JSON Data Size: {len(json_data)} bytes")

s.send(json_data.encode())