use chrono::Utc;

use crate::crc::crc16;

const START_MARKER: u8 = 0x0A;
const END_MARKER: u8 = 0x0D;

/// Only format currently supported (5.5.1.4). ADM-CID not yet implemented.
/// Will become an enum variant once other formats are added.
pub const SIA_DCS_TOKEN: &str = "SIA-DCS";

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

    let crc = crc16(body.as_bytes());
    let length = format!("{:04X}", body.len());

    let mut message = Vec::with_capacity(body.len() + 10);
    message.push(START_MARKER);
    message.extend(format!("{crc:04X}").into_bytes());
    message.extend(length.into_bytes());
    message.extend(body.into_bytes());
    message.push(END_MARKER);
    message
}
