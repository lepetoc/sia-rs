use sia_rs::{Account, Client};
use std::io::prelude::*;
use std::net::TcpStream;

const EXPECTED_ARGS: usize = 4;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != EXPECTED_ARGS {
        eprintln!("Usage: {} <compte> <ip:port> <code>", args[0]);
        eprintln!("Exemple: {} 56789 192.168.1.191:5555 NFA0001", args[0]);
        std::process::exit(1);
    }
    let account_number = &args[1];
    let address = &args[2];
    let code = &args[3];

    let account = Account::new(account_number, "0", None);
    let mut client = Client::new(account);
    let message = client.build_event(code);

    let mut stream = TcpStream::connect(address)?;
    stream.write_all(&message)?;

    let mut buffer = [0; 128];
    let bytes_read = stream.read(&mut buffer)?;
    println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
    Ok(())
}
