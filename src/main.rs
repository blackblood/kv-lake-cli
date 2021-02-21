use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

fn get_port_number(args: &mut Vec<String>) -> u32 {
    if args.is_empty() {
        println!("defaulting to port: 8000");
        8000
    } else {
        let port = match args[0].to_string().parse::<u32>() {
            Ok(p) => p,
            Err(_msg) => {
                println!("invalid port number. defaulting to 8000");
                8000
            }
        };
        args.remove(0);
        port
    }
}

fn main() -> io::Result<()> {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let mut port = 8000;
    if !args.is_empty() {
        port = get_port_number(&mut args);
    }

    let mut writer = TcpStream::connect(format!("localhost:{}", port))?;
    println!("connected to server at port: {}", port);
    let mut reader = writer.try_clone()?;

    loop {
        let mut input_str = String::new();
        io::stdin().read_line(&mut input_str)?;
        if let Err(_n) = writer.write(&[input_str.len() as u8]) {
        }
        if let Ok(_n) = writer.write(input_str.as_bytes()) {
            println!("Sent: {}", &input_str);
            writer.flush()?;
        }
        let mut input_length = [0; 1];
        reader.read(&mut input_length)?;
        let mut input_buffer: Vec<u8> = vec![0; input_length[0] as usize];
        reader.read(&mut input_buffer)?;
        let string_buffer = std::str::from_utf8(&input_buffer).unwrap();
        println!("response = {}", string_buffer);
        if string_buffer == "quit ack" {
            break;
        }
    }

    Ok(())
}
