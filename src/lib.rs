use chrono::Utc;

pub fn calculate_table() -> [u16; 256] {
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

pub fn crc16(data: &[u8], table: &[u16; 256]) -> u16 {
    let mut crc: u16 = 0;
    for &byte in data {
        let index = ((crc ^ byte as u16) & 0xFF) as usize;
        crc = (crc >> 8) ^ table[index];
    }
    crc
}

fn generate_timestamp() -> String {
    Utc::now().format("_%H:%M:%S,%m-%d-%Y").to_string()
}

pub fn build_message(
    table: &[u16; 256],
    id_token: &str,
    sequence: &str,
    account_line: &str,
    data_block: &str,
) -> Vec<u8> {
    let timestamp = generate_timestamp();
    let body = format!("\"{id_token}\"{sequence}{account_line}{data_block}{timestamp}");

    let crc = crc16(body.as_bytes(), table);
    let length = format!("{:04X}", body.len());

    let mut message = Vec::with_capacity(body.len() + 10);
    message.push(0x0A);
    message.extend(format!("{crc:04X}").into_bytes());
    message.extend(length.into_bytes());
    message.extend(body.into_bytes());
    message.push(0x0D);
    message
}
