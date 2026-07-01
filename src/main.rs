use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.1.191:5555")?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
    // 0x0A CRC 0LLL "SIA-DCS" 0001 L0#56789 [NFA0001] _HH:MM:SS,MM-DD-YYYY 0x0D
    // println!("{}", sia_rs::greet());
    // let lookup_table = calculate_table();
}

fn calculate_table() -> [u16; 256] {
    let mut table: [u16; 256] = [0; 256];
    for i in 0..=255 {
        let mut temp = i as u16;
        for _ in 0..8 {
            if temp & 1 == 0b1 {
                temp = temp >> 1;
                temp = temp ^ 0xA001;
            } else {
                temp = temp >> 1;
            }
        }
        table[i] = temp;
        println!("Ox{:04X}", temp);
    }
    table
}
