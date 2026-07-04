use chrono::{DateTime, Utc};

use crate::account::Account;
use crate::message::{SIA_DCS_TOKEN, build_message};

/// Entry point for building outgoing messages for one account.
/// Holds no connection: sending bytes stays the caller's responsibility.
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
        let sequence = self.account.next_sequence();
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
}
