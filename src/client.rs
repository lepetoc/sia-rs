use chrono::{DateTime, Utc};

use crate::account::Account;
use crate::message::{SIA_DCS_TOKEN, SUPERVISION_TOKEN, build_message};

/// Entry point for building outgoing messages for one account.
/// Holds no connection: sending bytes stays the caller's responsibility.
#[derive(Debug)]
pub struct Client {
    account: Account,
    now: fn() -> DateTime<Utc>,
}

impl Client {
    pub fn new(account: Account) -> Self {
        Client {
            account,
            now: Utc::now,
        }
    }

    /// Like `new`, but with an explicit time source for the message
    /// timestamps. Intended for tests and replay scenarios.
    pub fn with_clock(account: Account, now: fn() -> DateTime<Utc>) -> Self {
        Client { account, now }
    }

    /// Builds a complete SIA-DCS event message, advancing the sequence number.
    pub fn build_event(&mut self, code: &str) -> Vec<u8> {
        let sequence = format!("{:04}", self.account.next_sequence());
        let account_line = self.account.account_line();
        let data_block = self.account.data_block(code);
        build_message(
            SIA_DCS_TOKEN,
            &sequence,
            &account_line,
            &data_block,
            (self.now)(),
        )
    }

    /// Builds an unencrypted supervision (link test) message (5.5.2.1.1).
    /// Sequence is always "0000" and the data field always empty, per the
    /// standard — this does not advance the account's own sequence number,
    /// which is why this method only needs `&self`, not `&mut self`.
    pub fn build_supervision(&self) -> Vec<u8> {
        let account_line = self.account.account_line();
        build_message(SUPERVISION_TOKEN, "0000", &account_line, "[]", (self.now)())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn fixed_now() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap()
    }

    fn client() -> Client {
        let account = Account::new("1234", "0", None).unwrap();
        Client::with_clock(account, fixed_now)
    }

    #[test]
    fn build_event_produces_full_message() {
        let message = client().build_event("NFA0001");
        // Golden vector: CRC B222 / length 0037 computed independently.
        let expected = b"\x0AB2220037\"SIA-DCS\"0001L0#1234[#1234|NFA0001]_03:04:05,01-02-2024\x0D";
        assert_eq!(message, expected);
    }

    #[test]
    fn build_event_advances_sequence() {
        let mut client = client();
        client.build_event("NFA0001");
        let second = client.build_event("NFA0001");
        let expected = b"\x0A70640037\"SIA-DCS\"0002L0#1234[#1234|NFA0001]_03:04:05,01-02-2024\x0D";
        assert_eq!(second, expected);
    }

    #[test]
    fn new_uses_system_clock() {
        // Only checks framing, since the timestamp is live.
        let account = Account::new("1234", "0", None).unwrap();
        let message = Client::new(account).build_event("NFA0001");
        assert_eq!(message.first(), Some(&0x0A));
        assert_eq!(message.last(), Some(&0x0D));
    }

    #[test]
    fn build_supervision_uses_null_token_zero_sequence_empty_data() {
        let message = client().build_supervision();
        let text = String::from_utf8(message).unwrap();
        assert!(text.contains("\"NULL\"0000L0#1234[]"));
    }

    #[test]
    fn build_supervision_does_not_advance_account_sequence() {
        let mut client = client();
        client.build_supervision();
        client.build_supervision();
        // The next real event still gets sequence 0001 — supervision calls
        // never touched the account's own counter.
        let event = client.build_event("NFA0001");
        let text = String::from_utf8(event).unwrap();
        assert!(text.contains("\"SIA-DCS\"0001"));
    }
}
