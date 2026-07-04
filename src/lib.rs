use chrono::Utc;

const CRC_POLY: u16 = 0xA001;
const START_MARKER: u8 = 0x0A;
const END_MARKER: u8 = 0x0D;

/// Only format currently supported (5.5.1.4). ADM-CID not yet implemented.
/// Will become an enum variant once other formats are added.
pub const SIA_DCS_TOKEN: &str = "SIA-DCS";

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

pub struct Account {
    account_number: String,
    prefix: String,
    receiver: Option<String>,
    sequence: u16,
}

impl Account {
    pub fn new(account_number: &str, prefix: &str, receiver: Option<&str>) -> Self {
        Account {
            account_number: account_number.to_string(),
            prefix: prefix.to_string(),
            receiver: receiver.map(|r| r.to_string()),
            sequence: 0,
        }
    }

    /// Advances and returns the next sequence number as a 4-digit string.
    /// Wraps 9999 -> 0001, skipping the reserved 0000 (5.5.1.5).
    fn next_sequence(&mut self) -> String {
        self.sequence = if self.sequence >= 9999 {
            1
        } else {
            self.sequence + 1
        };
        format!("{:04}", self.sequence)
    }

    /// Builds Rrcvr+Lpref+#acct in the order required by the standard.
    /// Receiver is omitted entirely when absent, never sent as "R0" (5.5.1.6.3).
    fn account_line(&self) -> String {
        match &self.receiver {
            Some(r) => format!("R{r}L{}#{}", self.prefix, self.account_number),
            None => format!("L{}#{}", self.prefix, self.account_number),
        }
    }

    /// Builds the data field, repeating the account number as required (5.5.1.7).
    fn data_block(&self, code: &str) -> String {
        format!("[#{}|{code}]", self.account_number)
    }
}

/// Entry point for building outgoing messages for one account.
/// Holds no connection: sending bytes stays the caller's responsibility.
pub struct Client {
    account: Account,
}

impl Client {
    pub fn new(account: Account) -> Self {
        Client { account }
    }

    /// Builds a complete SIA-DCS event message, advancing the sequence number.
    pub fn build_event(&mut self, code: &str) -> Vec<u8> {
        let sequence = self.account.next_sequence();
        let account_line = self.account.account_line();
        let data_block = self.account.data_block(code);
        build_message(SIA_DCS_TOKEN, &sequence, &account_line, &data_block)
    }
}
