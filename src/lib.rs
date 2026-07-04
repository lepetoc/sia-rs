use chrono::Utc;

const CRC_POLY: u16 = 0xA001;
const START_MARKER: u8 = 0x0A;
const END_MARKER: u8 = 0x0D;

pub const fn calculate_table() -> [u16; 256] {
    let mut table: [u16; 256] = [0; 256];
    let mut i = 0;
    while i < 256 {
        let mut temp = i as u16;
        let mut j = 0;
        while j < 8 {
            let carry = temp & 1;
            temp >>= 1;
            if carry == 1 {
                temp ^= CRC_POLY;
            }
            j += 1;
        }
        table[i] = temp;
        i += 1;
    }
    table
}

pub const CRC_TABLE: [u16; 256] = calculate_table();

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
    id_token: &str,
    sequence: &str,
    account_line: &str,
    data_block: &str,
) -> Vec<u8> {
    let timestamp = generate_timestamp();
    let body = format!("\"{id_token}\"{sequence}{account_line}{data_block}{timestamp}");

    let crc = crc16(body.as_bytes(), &CRC_TABLE);
    let length = format!("{:04X}", body.len());

    let mut message = Vec::with_capacity(body.len() + 10);
    message.push(START_MARKER);
    message.extend(format!("{crc:04X}").into_bytes());
    message.extend(length.into_bytes());
    message.extend(body.into_bytes());
    message.push(END_MARKER);
    message
}
