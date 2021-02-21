use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut args: Vec<String> = std::env::args().collect();
    args.pop();
    let port = if args.is_empty() || args[0].is_empty() {
        println!("defaulting to port: 8000");
        8000
    } else {
        match args[1].to_string().parse::<u32>() {
            Ok(p) => p,
            Err(_msg) => {
                println!("invalid port number. defaulting to 8000");
                8000
            }
        }
    };
    let mut writer = TcpStream::connect(format!("localhost:{}", port))?;
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
