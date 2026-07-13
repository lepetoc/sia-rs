use chrono::{DateTime, Utc};

use crate::crc::crc16;

const START_MARKER: u8 = 0x0A;
const END_MARKER: u8 = 0x0D;

/// Only format currently supported (5.5.1.4). ADM-CID not yet implemented.
/// Will become an enum variant once other formats are added.
pub const SIA_DCS_TOKEN: &str = "SIA-DCS";

/// Token for the unencrypted supervision (link test) message (5.5.2.1.1).
/// Always paired with sequence "0000" and an empty data block.
pub const SUPERVISION_TOKEN: &str = "NULL";

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn fixed_timestamp() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap()
    }

    #[test]
    fn body_contains_all_fields_in_order() {
        let body = build_body(
            SIA_DCS_TOKEN,
            "0001",
            "L0#1234",
            "[#1234|NFA0001]",
            fixed_timestamp(),
        );
        assert_eq!(
            body,
            "\"SIA-DCS\"0001L0#1234[#1234|NFA0001]_03:04:05,01-02-2024"
        );
    }

    #[test]
    fn timestamp_is_zero_padded_utc() {
        let body = build_body(
            "",
            "",
            "",
            "",
            Utc.with_ymd_and_hms(2025, 12, 31, 23, 59, 9).unwrap(),
        );
        assert_eq!(body, "\"\"_23:59:09,12-31-2025");
    }

    #[test]
    fn frame_wraps_body_with_markers_crc_and_length() {
        // CRC and length computed with an independent implementation.
        let framed = frame("123456789");
        let mut expected = vec![0x0A];
        expected.extend(b"BB3D");
        expected.extend(b"0009");
        expected.extend(b"123456789");
        expected.push(0x0D);
        assert_eq!(framed, expected);
    }

    #[test]
    fn frame_length_field_is_hexadecimal() {
        let body = "x".repeat(255);
        let framed = frame(&body);
        // Skip start marker (1) and CRC (4): the next 4 bytes are the length.
        assert_eq!(&framed[5..9], b"00FF");
    }

    #[test]
    fn build_message_composes_body_and_frame() {
        let message = build_message(
            SIA_DCS_TOKEN,
            "0001",
            "L0#1234",
            "[#1234|NFA0001]",
            fixed_timestamp(),
        );
        // Golden vector: CRC B222 / length 0037 computed independently.
        let expected = b"\x0AB2220037\"SIA-DCS\"0001L0#1234[#1234|NFA0001]_03:04:05,01-02-2024\x0D";
        assert_eq!(message, expected);
    }
}
