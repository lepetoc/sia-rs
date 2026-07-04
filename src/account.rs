/// Single generic error for invalid Account fields.
#[derive(Debug)]
pub struct AccountError(String);

impl std::fmt::Display for AccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AccountError {}

/// A PE account: identity fields plus the running sequence number (5.5.1.5, 5.5.1.6).
pub struct Account {
    account_number: String,
    prefix: String,
    receiver: Option<String>,
    sequence: u16,
}

impl Account {
    /// Validates account_number (3-16 hex digits, 5.5.1.6.1) before construction.
    /// Prefix and receiver are not yet validated.
    pub fn new(
        account_number: &str,
        prefix: &str,
        receiver: Option<&str>,
    ) -> Result<Self, AccountError> {
        let len = account_number.len();
        if !(3..=16).contains(&len) {
            return Err(AccountError(format!(
                "account number must be 3-16 characters, got {len}"
            )));
        }
        if !account_number.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(AccountError(format!(
                "account number must contain only hexadecimal digits, got '{account_number}'"
            )));
        }

        Ok(Account {
            account_number: account_number.to_string(),
            prefix: prefix.to_string(),
            receiver: receiver.map(|r| r.to_string()),
            sequence: 0,
        })
    }

    /// Advances and returns the next sequence number as a 4-digit string.
    /// Wraps 9999 -> 0001, skipping the reserved 0000 (5.5.1.5).
    pub(crate) fn next_sequence(&mut self) -> String {
        self.sequence = if self.sequence >= 9999 {
            1
        } else {
            self.sequence + 1
        };
        format!("{:04}", self.sequence)
    }

    /// Builds Rrcvr+Lpref+#acct in the order required by the standard.
    /// Receiver is omitted entirely when absent, never sent as "R0" (5.5.1.6.3).
    pub(crate) fn account_line(&self) -> String {
        match &self.receiver {
            Some(r) => format!("R{r}L{}#{}", self.prefix, self.account_number),
            None => format!("L{}#{}", self.prefix, self.account_number),
        }
    }

    /// Builds the data field, repeating the account number as required (5.5.1.7).
    pub(crate) fn data_block(&self, code: &str) -> String {
        format!("[#{}|{code}]", self.account_number)
    }
}
