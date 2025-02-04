import socket
import json
import time

def MoveMouse(socket : socket.socket):
	x = input("Input X: ")
	y = input("Input Y: ")
 
	data = { "name" : "move_mouse", "args" : [x,y]}
	msg = json.dumps(data).encode()
	socket.send(msg);
 
def Type(socket : socket.socket):

 
	data = { "name" : "type", "args" : [input("Type Message:\n").lower()]}
	msg = json.dumps(data).encode()
	socket.send(msg);
 
def Scroll(socket : socket.socket):

 
	data = { "name" : "mouse_scroll", "args" : [input("Enter Scroll Ammount: (positive or negative)\n").lower()]}
	msg = json.dumps(data).encode()
	socket.send(msg);
 
def SpecialKeyToggle(socket : socket.socket):

 
	data = { "name" : "special_key_toggle", "args" : [input("Enter Key to toggle: (capslock)\n").lower()]}
	msg = json.dumps(data).encode()
	socket.send(msg);
 
def MouseClick(socket : socket.socket):

 
	data = { "name" : "mouse_click", "args" : [input("Enter Key to click (left or right):\n").lower()]}
	msg = json.dumps(data).encode()
	socket.send(msg);
 
def FunctionKey(socket : socket.socket):
	data = { "name" : "function_key_click", "args" : [input("Enter function key (1,2,3...11,12):\n").lower()]}
	msg = json.dumps(data).encode()
	socket.send(msg);


def MediaControl(socket : socket.socket):
	data = { "name" : "media_key_click", "args" : [input("Enter media key (play, next, previous, stop, mute_volume):\n").lower()]}
	msg = json.dumps(data).encode()
	socket.send(msg);


def VolumeControl(socket : socket.socket):
	data = { "name" : "volume_control", "args" : [input("Enter ammount to change volume key (postive or negative):\n").lower()]}
	msg = json.dumps(data).encode()
	socket.send(msg);

def KeyHold(socket : socket.socket):
	data = { "name" : "toggle_key_hold", "args" : [input("Enter key to hold (shift):\n").lower()]}
	msg = json.dumps(data).encode()
	socket.send(msg);
 
def LimitTest(socket : socket.socket):
	with open('LimitTest.txt','r') as file:
		data = file.read()

	data_send= {"name" : "type", "args" : [""] }
	data_send["args"] = [data]
	msg = json.dumps(data_send).encode()
	socket.send(msg);
 
def InvalidJson(socket : socket.socket):
	data = '{aaa :}'

	msg = json.dumps(data).encode()
	socket.send(msg);
 
def connect_to_server():
    # Define the server address and port
    host = "127.0.0.1"
    port = 8080

    """"""
    
    # data = 
    # {
    #     "name": "type",
    #     "args" : ["texthere"] # \n to type new line \t to press tab /x08 to press backspace
    # }
    
    # data = 
    # {
    #     "name": "move_mouse",
    #     "args" : ["x","y"]
    # }
    
    # data = 
    # {
    #     "name": "mouse scroll",
    #     "args" : ["y"] #left or right
    # }
    
    
    # data = 
    # {
    #     "name": "special_key_press",
    #     "args" : ["capslock"]
    # }
    
    # data = 
    # {
    #     "name": "mouse_click",
    #     "args" : ["left"] #left or right
    # }
    
    # data = 
    # {
    #     "name": "volume_control",
    #     "args" : ["x"] 
    # }
    
    # data = 
    # {
    #     "name": "special_key_click",
    #     "args" : ["x"] #"play", "next", "previous", "stop", "mute_volume"
    # }
    
    # data = 
    # {
    #     "name": "function_key_click",
    #     "args" : ["x"] #"play", "next", "previous", "stop", "mute_volume"
    # }
    
     # data = 
    # {
    #     "name": "function_toggle_key_hold",
    #     "args" : ["x"] #"play", "next", "previous", "stop", "mute_volume"
    # }
    
    """"""
    
    
    
    # Create a socket object
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        try:
            # Connect the socket to the server
            print(f"Connecting to {host}:{port}...")
            client_socket.connect((host, port))
            print(f"Connected to {host}:{port}")
            
            
            print(type(client_socket))
            
            while True:
                print("""
                      Options:
                      
                      1: Move Mouse (x,y)
                      2: Type (input)
                      3: Scroll (x)
                      4: Special Key (capslock)
                      5: Mouse Click (left or right)
                      6: Volume Control (x)
                      7: Media Key Click ("play", "next", "previous", "stop", "mute_volume")
                      8: Function Key Click (1-12)
                      9: Toggle Key Held (Shift)
                      10: Send Limit Test 1 (Bee movie script)
                      11: Send Invalid Json
                      """)
                
                
                
                
                inp = int(input("Enter option: "))
                
                match inp:
                    case 1:
                        MoveMouse(client_socket)
                    case 2:
                        Type(client_socket)
                    case 3:
                        Scroll(client_socket)
                        pass
                    case 4:
                        SpecialKeyToggle(client_socket)
                        pass
                    case 5:
                        MouseClick(client_socket)
                        pass
                    case 6:
                        VolumeControl(client_socket)
                        pass
                    case 7:
                        MediaControl(client_socket)
                        pass
                    case 8:
                        FunctionKey(client_socket)
                        pass
                    case 9:
                        SpecialKeyToggle(client_socket)
                        pass
                    case 10:
                        LimitTest(client_socket)
                        pass
                    case 11:
                        InvalidJson(client_socket)
                        
                    case 0 : break
                    case _:
                        pass
            client_socket.close()

        except Exception as e:
            print(f"An error occurred: {e}")
            

if __name__ == "__main__":
    connect_to_server()
    
    """Hello Wold. Oops yped wo peiods!
    Hello Wold. Oops yped wo peiods!
    Hello World. Oops typed two periods!
    Hello World. Oops typed two pe	riods!
    
    """
    
    
    

    