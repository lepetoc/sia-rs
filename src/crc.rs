const CRC_POLY: u16 = 0xA001;

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

pub fn crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    for &byte in data {
        let index = ((crc ^ byte as u16) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC_TABLE[index];
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_is_zero() {
        assert_eq!(crc16(b""), 0x0000);
    }

    #[test]
    fn matches_crc16_arc_check_value() {
        // Standard check value for CRC-16/ARC (poly 0xA001 reflected, init 0).
        assert_eq!(crc16(b"123456789"), 0xBB3D);
    }

    #[test]
    fn table_first_entries() {
        assert_eq!(CRC_TABLE[0], 0x0000);
        assert_eq!(CRC_TABLE[1], 0xC0C1);
        assert_eq!(CRC_TABLE[255], 0x4040);
    }
}
