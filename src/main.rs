mod window_controller;

use std::vec;

use enigo::{
    Axis, Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use once_cell::sync::Lazy;
use serde::{
    Deserialize, Serialize,
};
use serde_json::Error as JsonError;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;


#[derive(Serialize, Deserialize, Debug)]
struct Command {
    name: String,
    args: Vec<String>,
}
// 0 = shift 1 = control 2 = alt
static SPECIAL_CHARS: Lazy<Vec<char>> = Lazy::new(|| vec!['\t', '\n', '\r', '\x08']);
static MAX_DATA_SIZE : usize = 16384;
#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    let port = "127.0.0.1:8080";



    let listener = TcpListener::bind(port).await.expect("Failed to bind");
    println!("Listening on {}", port);
    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                tokio::spawn(async move {
                    println!("Accepting new Connection");
                    handle_connection(socket).await
                });
            }
            Err(e) => {
                println!("Failed to accept connection: {}", e);
            }
        }
    }
}

async fn handle_connection(mut socket: tokio::net::TcpStream) -> io::Result<()> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut key_held: [bool; 3] = [false, false, false]; // shitft, alt, control
    let mut window_controller = window_controller::WindowController::new("Windows".to_string());
    println!("Socket Opened");
    
    let mut buf = vec![0u8; 3072];
    
    let mut data: Vec<u8> = Vec::new();

    let mut stop_reading : bool = false;
    
    let mut command: Command;
    
    loop {
        loop {
            
            if stop_reading
            {
                println!("Pausing data reception temporarily.");
                let mut trash = vec![0u8; 1024];

                loop {
                    match  socket.try_read(&mut trash) {
                        Ok(0) =>
                        {
                            println!("Socket Flushed!");
                            break;
                        }
                        Ok(n) =>
                        {

                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock =>
                        {
                            println!("Socket Flushed!");
                            break;
                        }
                        Err(e) =>
                        {
                            println!("{}\nContinuing!",e)
                        }
                    }
                }

                stop_reading = false;
                continue;

            }
            
            let n = socket.read(&mut buf).await?;
            if n == 0 {
                println!("Connection Closed");
				return  Ok(());
            }

            data.extend_from_slice(&buf[..n]);
            match parse_command(&data) {
                Ok(c) => {
                    command = c;
                    break;
                }
				
                Err(e) => {

                    if data.len() > MAX_DATA_SIZE
                    {
                        println!("Sent data is too large. Clearing buffer {}",e);
                        stop_reading = true;

                        data.clear();
                    }
					else if e.to_string().contains("EOF")
					{
						println!("Reading more data");
					}
					else {
						data.clear();
						println!("Invalid Command. Continuing {}",e);
					}
                    // return Ok(());
                }
            };
        }
        handle_command(command, &mut enigo, &mut key_held, &mut window_controller, &mut socket)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
		data.clear();
    }
}

fn parse_command(data: &[u8]) -> Result<Command, JsonError> {
    let command_str = String::from_utf8_lossy(data).to_string();
	// println!("Parsed : {}",command_str);
    serde_json::from_str(&command_str)
}

async fn handle_command( command: Command, enigo: &mut Enigo, key_held: &mut [bool; 3], window_controller : &mut window_controller::WindowController, socket : &mut tokio::net::TcpStream) -> Result<(), String> {
    match command.name.to_lowercase().as_str() {
        "add" => {
            let sum: i32 = command
                .args
                .iter()
                .filter_map(|val| val.parse::<i32>().ok())
                .sum();

            println!("The sum is {}!", sum);
            Ok(())
        }

        "move_mouse" => {
            println!("Moving Mouse");

            handle_mouse_move(
                command.args[0].parse::<i32>().unwrap_or(0),
                command.args[1].parse::<i32>().unwrap_or(0),
                enigo,
            )
            .await;

            Ok(())
        }

        "mouse_click" => {
            println!("Mouse Click");

            handle_mouse_click(&command.args[0], enigo).await;
            Ok(())
        }

        "mouse_scroll" => {
            println!("Mouse Click");

            handle_scroll(command.args[0].parse::<i32>().unwrap_or(0), enigo).await;
            Ok(())
        }

        "type" => {
            // println!("Typing {}", &command.args[0]);

            handle_typing(&command.args[0], enigo).await;

            Ok(())
        }

        "volume_control" => {
            println!("Controling Volume");
            handle_volume_control(command.args[0].parse::<i32>().unwrap_or(0), enigo).await;
            Ok(())
        }

        "special_key_toggle" => {
            println!("Toggling {}", &command.args[0]);
            handle_special_key_toggle(&command.args[0], enigo).await;
            Ok(())
        }

        "media_key_click" => {
            println!("CLicking {}", &command.args[0]);
            handle_media_key_click(&command.args[0], enigo);
            Ok(())
        }

        "function_key_click" => {
            println!("CLicking F{}", &command.args[0]);
            handle_function_key_click(command.args[0].parse::<i32>().unwrap_or(0), enigo);
            Ok(())
        }

        "toggle_key_hold" => {
            println!("Toggling {}", &command.args[0]);
            handle_toggle_key_hold(&command.args[0], enigo, key_held);
            Ok(())
        }

        "get_open_windows" =>
        {
            handle_get_open_window(window_controller,socket).await;

            Ok(())
        }

		"set_active_window" =>
		{
			handle_set_active_window(window_controller, command.args[0].parse::<i32>().unwrap_or(0)).await;
			Ok(())
		}

        _ => {
            println!("{} is not a valid command!", command.name);
            Err(format!("{} is not a valid command!", command.name))
        }
    }
}

async fn handle_mouse_move(x: i32, y: i32, enigo: &mut Enigo) {
    let mut t: f32 = 0.05;

    let delay = std::time::Duration::from_millis(5);

    while t <= 1.0 {
        let ease = quadratic_ease_in_out(t);

        let _ = enigo.move_mouse(
            (x as f32 * ease) as i32,
            (y as f32 * ease) as i32,
            Coordinate::Rel,
        );

        t += 0.05;

        tokio::time::sleep(delay).await;
    }
}

fn quadratic_ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        return 2.0 * t * t;
    } else {
        return -1.0 + (4.0 - 2.0 * t) * t;
    }
}

async fn handle_typing(input: &String, enigo: &mut Enigo) {
    let delay = std::time::Duration::from_millis(10);

    for c in input.chars() {
        if SPECIAL_CHARS.contains(&c) {
            handle_special_char(&c, enigo);
        } else {
            let _ = enigo.text(&c.to_string());
        }

        tokio::time::sleep(delay).await;
    }
}
async fn handle_special_key_toggle(key: &String, enigo: &mut Enigo) {
    match key.to_lowercase().as_str() {
        "capslock" => {
            println!("Toggling Caps Lock!");
            let _ = enigo.key(Key::CapsLock, Click);
        }

        _ => {}
    }
}

fn handle_special_char(c: &char, enigo: &mut Enigo) {
    match c {
        '\x08' => {
            let _ = enigo.key(Key::Backspace, Click);
        }
        '\n' => {
            let _ = enigo.key(Key::Return, Click);
        }
        '\t' => {
            let _ = enigo.key(Key::Tab, Click);
        }
        _ => {
            println!("Unknown Special Character!");
        }
    }
}

async fn handle_mouse_click(input: &String, enigo: &mut Enigo) {
    match input.to_lowercase().as_str() {
        "left" => {
            let _ = enigo.button(Button::Left, Click);
        }

        "right" => {
            let _ = enigo.button(Button::Right, Click);
        }

        _ => println!("Unknown Button!"),
    }
}

async fn handle_scroll(amt: i32, enigo: &mut Enigo) {
    let direction = if amt > 0 {
        1
    } else if amt < 0 {
        -1
    } else {
        0
    };

    let delay = std::time::Duration::from_millis(10);
    for _ in 0..amt.abs() {
        let _ = enigo.scroll(1 * direction, Axis::Vertical);

        tokio::time::sleep(delay).await;
    }
}
async fn handle_volume_control(amt: i32, enigo: &mut Enigo) {
    let volume_direction: Key = if amt > 0 {
        Key::VolumeUp
    } else {
        Key::VolumeDown
    };

    let delay = std::time::Duration::from_millis(10);
    for _ in 0..amt.abs() {
        let _ = enigo.key(volume_direction, Click);
        tokio::time::sleep(delay).await;
    }
}

fn handle_media_key_click(input: &str, enigo: &mut Enigo) {
    match input.to_lowercase().as_str() {
        "play" => {
            let _ = enigo.key(Key::MediaPlayPause, Click);
        }

        "next" => {
            let _ = enigo.key(Key::MediaNextTrack, Click);
        }
        "previous" => {
            let _ = enigo.key(Key::MediaPrevTrack, Click);
        }
        "stop" => {
            let _ = enigo.key(Key::MediaStop, Click);
        }
        "mute_volume" => {
            let _ = enigo.key(Key::VolumeMute, Click);
        }

        _ => {
            println!("Unknown Special Key")
        }
    }
}

fn handle_function_key_click(input: i32, enigo: &mut Enigo) {
    match input {
        1 => {
            let _ = enigo.key(Key::F1, Click);
        }

        2 => {
            let _ = enigo.key(Key::F2, Click);
        }
        3 => {
            let _ = enigo.key(Key::F3, Click);
        }

        4 => {
            let _ = enigo.key(Key::F4, Click);
        }
        5 => {
            let _ = enigo.key(Key::F5, Click);
        }
        6 => {
            let _ = enigo.key(Key::F6, Click);
        }
        7 => {
            let _ = enigo.key(Key::F7, Click);
        }
        8 => {
            let _ = enigo.key(Key::F8, Click);
        }
        9 => {
            let _ = enigo.key(Key::F9, Click);
        }
        10 => {
            let _ = enigo.key(Key::F10, Click);
        }
        11 => {
            let _ = enigo.key(Key::F11, Click);
        }
        12 => {
            let _ = enigo.key(Key::F12, Click);
        }

        _ => {
            println!("Unknown Special Key")
        }
    }
}

fn handle_toggle_key_hold(input: &str, enigo: &mut Enigo, key_held: &mut [bool; 3]) {
    match input.to_lowercase().as_str() {
        "shift" => {
            let dir = if key_held[0] { Release } else { Press };

            key_held[0] = !key_held[0];

            let _ = enigo.key(Key::Shift, dir);
        }

        _ => {
            println!("Unknown Key")
        }
    }
}

async fn handle_get_open_window(window_controller : &mut window_controller::WindowController, socket : &mut tokio::net::TcpStream)
{
	let _ = window_controller.get_open_windows();
	let msg: String = window_controller
    .open_windows
    .iter()
    .enumerate()
    .map(|(i, w)| format!("\n{}: {}", i, w.title))
    .collect();

	if let Err(e) = socket.write_all(msg.as_bytes()).await {
        eprintln!("Failed to send message to client: {}", e);
    }
}
async fn handle_set_active_window(window_controller : &mut window_controller::WindowController, i : i32)
{
	window_controller.set_active_window(i);
}