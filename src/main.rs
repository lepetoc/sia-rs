use chrono::Utc;
use std::io::prelude::*;
use std::net::TcpStream;

const SEQUENCE: &str = "0001";
const ACCOUNT_LINE: &str = "L0#56789";
const DATA_BLOCK: &str = "[#56789|NFA0001]";

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.1.191:5555")?;

    let message = build_message();
    stream.write_all(&message)?;

    let mut buffer = [0; 128];
    let bytes_read = stream.read(&mut buffer)?;
    println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));

    Ok(())
}

fn build_message() -> Vec<u8> {
    let table = calculate_table();

    let timestamp = Utc::now().format("_%H:%M:%S,%m-%d-%Y").to_string();
    let body = format!("\"SIA-DCS\"{SEQUENCE}{ACCOUNT_LINE}{DATA_BLOCK}{timestamp}");

    let crc = crc16(body.as_bytes(), &table);
    let length = format!("{:04X}", body.len());

    let mut message = Vec::with_capacity(body.len() + 11);
    message.push(0x0A);
    message.extend(format!("{crc:04X}").into_bytes());
    message.extend(length.into_bytes());
    message.extend(body.into_bytes());
    message.push(0x0D);
    message
}

fn crc16(data: &[u8], table: &[u16; 256]) -> u16 {
    let mut crc: u16 = 0;
    for &byte in data {
        let index = ((crc ^ byte as u16) & 0xFF) as usize;
        crc = (crc >> 8) ^ table[index];
    }
    crc
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
    }
    table
}
