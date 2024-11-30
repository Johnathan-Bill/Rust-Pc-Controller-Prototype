
use std::vec;

use enigo::{
    Axis, Button, Coordinate, Direction::{Click, Press, Release}, Enigo, Key, Keyboard, Mouse, Settings
};
use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt,}; 
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    name: String,
    args: Vec<String>
}

static SPECIAL_CHARS : Lazy<Vec<char>> = Lazy::new(||
{
    vec!['\t','\n','\r','\x08']
});

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    let port = "127.0.0.1::8080";
    let listener = TcpListener::bind(port).await.expect("Failed to bind");
    println!("Listening on {}",port);
    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                

                tokio::spawn( async move{
                    println!("Accepting new Connection");
                    handle_connection(socket).await});
            },
            Err(e) => {
                println!("Failed to accept connection: {}", e);
        }
            
        }
    }
}

async fn handle_connection(mut socket: tokio::net::TcpStream) -> io::Result<()>
{
println!("Socket Opened");


loop {
    
// socket.write("TEST".as_bytes()).await;
let mut buf = Vec::new();

        let  n = socket.read_to_end(&mut buf).await?;



if n == 0 
{
    println!("Connection closed");
    return Ok(());
}
    
println!("{}", buf.len());

let com_str =  match String::from_utf8(buf) {
    Ok(s) => s,
    Err(_) =>
    {
        println!("Could not parse message to string");
        return Ok(());
    }
};


let command = match serde_json::from_str::<Command>(&com_str)
{
    Ok(s) => s,
    Err(_) =>
    {
        println!("Invalid Message (Must be Json)");
        return  Ok(());
    }
};

handle_command(command).await
.map_err(|e|std::io::Error::new(std::io::ErrorKind::Other, e))?;

}
}



async fn handle_command(command : Command) -> Result<(), String>
{

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

    "move_mouse" =>
    {

        println!("Moving Mouse");
        

        handle_mouse_move(command.args[0].parse::<i32>().unwrap_or(0),command.args[1].parse::<i32>().unwrap_or(0)).await;
        

    
        Ok(())
    }

    "mouse_click" =>
    {
        println!("Mouse Click");
        

        handle_mouse_click(&command.args[0]).await;
        Ok(())
    }

    "mouse_scroll" =>
    {
        println!("Mouse Click");

        handle_scroll(command.args[0].parse::<i32>().unwrap_or(0)).await;
        Ok(())
    }


    "type" =>
    {

        println!("Typing {}" ,&command.args[0]);

        handle_typing(&command.args[0]).await;


        Ok(())
    }


	"volume_control" =>
    {
        println!("Controling Volume");
        handle_volume_control(command.args[0].parse::<i32>().unwrap_or(0)).await;
        Ok(())
    }

    "special_key_toggle" =>
    {
        println!("Toggling {}" ,&command.args[0]);
        handle_special_key_toggle(&command.args[0]).await;
        Ok(())
    }

    _ =>
    {
        println!("{} is not a valid command!", command.name );
        Err(format!("{} is not a valid command!", command.name ))
    }
}        
    }



async fn handle_mouse_move(x : i32 , y : i32)
{
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut t: f32 = 0.05;

    let delay = std::time::Duration::from_millis(5);

    while t <= 1.0 
    {
    let ease = quadratic_ease_in_out(t);

    let _ = enigo.move_mouse(
        ((x as f32*ease)) as i32,
         ((y as f32*ease)) as i32,
         Coordinate::Rel);

    t += 0.05;

    tokio::time::sleep(delay).await;
    }

}

fn quadratic_ease_in_out(t : f32) -> f32
{
    if t < 0.5 {
        return 2.0 * t * t;
    } else {
        return -1.0 + (4.0 - 2.0 * t) * t;
    }
}

async fn handle_typing(input : &String)
{
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
        
    let delay = std::time::Duration::from_millis(10);
    
    for c in input.chars()
    {
        if SPECIAL_CHARS.contains(&c)
        {
            handle_special_char(&c);
        }
        else
        {
            let _ = enigo.text(&c.to_string());
            
        }

        tokio::time::sleep(delay).await;
    }
    
}
async fn handle_special_key_toggle(key : &String)
{
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    match key.as_str() {
        "capslock" =>
        {
            println!("Toggling Caps Lock!");
            let _ = enigo.key(Key::CapsLock, Click);
        }

        _ =>
        {
            
        }
    }
}

fn handle_special_char(c : &char)
{
    let mut enigo = Enigo::new(&Settings::default()).unwrap();


    match c 
    {
        '\x08' =>
        {
            let _ = enigo.key(Key::Backspace, Click);
        }
        '\n' =>
        {
            let _ = enigo.key(Key::Return, Click);
        }
        '\t' =>
        {
            
            let _ = enigo.key(Key::Tab, Click);

        }
        _ =>
        {
            println!("Unknown Special Character!");
        }
    }
}

async fn handle_mouse_click(input : &String)
{
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    match input.to_lowercase().as_str()
    {

        "left" =>
        {
            let _ = enigo.button(Button::Left, Click);
        }

        "right" => 
        {
            let _ = enigo.button(Button::Right, Click);
        }

        _ => println!("Unknown Button!")
    }
}

async fn handle_scroll(amt : i32)
{
    let direction = if amt > 0 {1} else if amt < 0 {-1} else {0};

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let delay = std::time::Duration::from_millis(10);
    for _ in 0..amt.abs()
    {
        let _ = enigo.scroll(1*direction, Axis::Vertical);

        tokio::time::sleep(delay).await;
    }
}
async fn handle_volume_control(amt : i32)
{
	let mut enigo = Enigo::new(&Settings::default()).unwrap();

	let volume_direction : Key = if amt > 0 {Key::VolumeUp} else {Key::VolumeDown};

	let delay = std::time::Duration::from_millis(10);
	for _ in 0..amt.abs()
	{
		let _ = enigo.key(volume_direction, Click);
		tokio::time::sleep(delay).await;
	}
}