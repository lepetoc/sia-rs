#[derive(Debug, PartialEq, Eq)]
pub enum ResponseError {
    Duh,
    Nak,
    Unknown(String),
}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::Duh => write!(f, "CSR returned DUH: received but not processed"),
            ResponseError::Nak => write!(f, "CSR returned NAK: message rejected"),
            ResponseError::Unknown(code) => write!(f, "unrecognized response: {code}"),
        }
    }
}
impl std::error::Error for ResponseError {}

fn extract_code(bytes: &[u8]) -> Option<&str> {
    let start = bytes.iter().position(|&b| b == b'"')? + 1;
    let end = bytes[start..].iter().position(|&b| b == b'"')? + start;
    std::str::from_utf8(&bytes[start..end]).ok()
}

pub fn check_response(bytes: &[u8]) -> Result<(), ResponseError> {
    match extract_code(bytes) {
        Some("ACK") => Ok(()),
        Some("DUH") => Err(ResponseError::Duh),
        Some("NAK") => Err(ResponseError::Nak),
        Some(other) => Err(ResponseError::Unknown(other.to_string())),
        None => Err(ResponseError::Unknown(String::new())),
    }
}
