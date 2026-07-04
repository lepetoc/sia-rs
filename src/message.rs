use chrono::{DateTime, Utc};

use crate::crc::crc16;

const START_MARKER: u8 = 0x0A;
const END_MARKER: u8 = 0x0D;

/// Only format currently supported (5.5.1.4). ADM-CID not yet implemented.
/// Will become an enum variant once other formats are added.
pub const SIA_DCS_TOKEN: &str = "SIA-DCS";

fn format_timestamp(timestamp: DateTime<Utc>) -> String {
    timestamp.format("_%H:%M:%S,%m-%d-%Y").to_string()
}

/// Assembles the message body: everything between the length field and the
/// end marker, i.e. the bytes covered by the CRC.
pub fn build_body(
    id_token: &str,
    sequence: &str,
    account_line: &str,
    data_block: &str,
    timestamp: DateTime<Utc>,
) -> String {
    let timestamp = format_timestamp(timestamp);
    format!("\"{id_token}\"{sequence}{account_line}{data_block}{timestamp}")
}

/// Wraps a body with the transport framing: start marker, CRC, length,
/// end marker (5.5.1.2, 5.5.1.3).
pub fn frame(body: &str) -> Vec<u8> {
    let crc = crc16(body.as_bytes());
    let length = format!("{:04X}", body.len());

    let mut message = Vec::with_capacity(body.len() + 10);
    message.push(START_MARKER);
    message.extend(format!("{crc:04X}").into_bytes());
    message.extend(length.into_bytes());
    message.extend(body.as_bytes());
    message.push(END_MARKER);
    message
}

pub fn build_message(
    id_token: &str,
    sequence: &str,
    account_line: &str,
    data_block: &str,
    timestamp: DateTime<Utc>,
) -> Vec<u8> {
    frame(&build_body(
        id_token,
        sequence,
        account_line,
        data_block,
        timestamp,
    ))
}
