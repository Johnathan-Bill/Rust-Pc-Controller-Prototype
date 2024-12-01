# Rust Pc Controller Prototype
This program, written in Rust, listens on a local socket and processes incoming JSON data to control mouse movements, clicks, scrolling, and typing. It is powered by asynchronous programming using tokio and utilizes the serde library for JSON deserialization. The program is designed to be a flexible tool for controlling a mouse through socket commands, with options to handle special key toggles and keyboard inputs.

## Why
This project is a prototype of a socket server designed to integrate with a Tauri-based React application. The goal is to build a seamless connection between the desktop application and a mobile companion app built with React Native.

The mobile app will translate touch controls into commands, which will then be sent to and received by the local host. This setup allows the user to interact with the desktop application through their mobile device.

## Dependencies
The following dependencies are used in the project:

- tokio = { version = "1", features = ["full"] }: Asynchronous runtime for Rust, enabling the handling of sockets and I/O operations.
- serde = { version = "1.0", features = ["derive"] }: Framework for serializing and deserializing Rust data structures.
- serde_json = "1.0": JSON serialization and deserialization library for serde.
- enigo = "0.3.0": A library for simulating mouse and keyboard events.
- once_cell = "1.20.2": A utility for lazy static initialization.
## Setup
Ensure you have Rust and Cargo installed on your system.

### Clone this repository:
```
git clone https://github.com/yourusername/rust-socket-json-mouse-control.git cd rust-socket-json-mouse-control
```

### Install dependencies:
```
cargo build
```

### Run the application:
```
cargo run
```

By default, the program listens on localhost and processes incoming data on a specified port.

## Commands & Usage
The program expects JSON data to be received over a socket. Each JSON object should contain a name key, representing the function to execute, and an args array, representing the arguments passed to that function. Here are the available commands:

move_mouse
- Args: ```"x" "y"```
  - Moves the mouse by x and y coordinates, where the movement is relative to the current mouse location.

Example:
```
{
"name": "move_mouse",
"args": ["100", "50"]
}
```

mouse_click
- Args: ```"button"```
  > Options : ("left", "right")
  - Clicks the specified mouse button (left or right).
Example:
```
{
"name": "mouse_click",
"args": ["left"]
 }
```

mouse_scroll
- Args: ```"x"```
-- Scrolls the mouse by x steps, where each x is equivalent to a 15-degree rotation of the mouse wheel.

Example:
```
{
"name": "mouse_scroll",
"args": [3]
 }
```

type
- Args: ```"input"```
  - Types the string specified in input.
  - Special Characters :
    - \x08 = Backspace
    - \n = Return
    - \t = tab
Example:

```
{
"name": "type",
"args": ["Hello, World!\nThis is a test.\x08"]
}
```

special_key_toggle
- Args:  ```"key"```
  - Toggles the specified special key (e.g., capslock).
  > Options : "capslock"

Example:
```
{
"name": "special_key_toggle",
"args": ["capslock"]
}
```

volume_control
- Args:  ```"x"```
  - Increases volume by specified ammount

Example:
```
{
"name": "volume_control",
"args": ["12"]
}
```

media_key_click
- Args:  ```"key"```
  - Clicks the specified special key (e.g., MediaPlay).
  > Options : "play", "next", "previous", "stop", "mute_volume"

Example:
```
{
"name": "special_key_click",
"args": ["play"]
}
```

function_key_click
- Args:  ```"key"```
  - Clicks the specified special key (e.g., 11).
  > Options : "1", "2", "3", "4", "5", "6", "7", "8", "9", "10","11","12"

Example:
```
{
"name": "function_key_click",
"args": ["11"]
}
```



## Example
To send a command to the program, you can use a tool like netcat or telnet to send a JSON string over a socket:
```
echo '{"name": "move_mouse", "args": [100, 50]}' | nc -u -w1 localhost 8080
```
This will move the mouse by 100 units along the X-axis and 50 units along the Y-axis.

## Planned Features
- [x] Mouse Movement
- [x] Mouse Button Iteration
- [x] Mouse Scrolling
- [x] Typing
- [x] Toggling Caps Lock
- [x] Volume Control
- [x] Media Controls
- [x] Function Keys
- [ ] Holding Keys (Shift, Alt, Ctrl etc..)
- [ ] Window Control
- [ ] Bluetooth capabilites


## Notes
The program uses asynchronous Rust code powered by tokio, so it is designed to handle multiple concurrent connections.
Ensure that the port the program is listening on is not blocked by a firewall.
The mouse control actions are designed to be basic and do not include advanced features like acceleration or boundary detection.
