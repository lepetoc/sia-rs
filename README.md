# sia-rs

A Rust implementation of the transmitter (premises equipment) side of the
[SIA DC-09](https://www.securityindustry.org/) protocol — the standard used
by alarm panels to report events to a central monitoring station over IP.

![CI](https://github.com/lepetoc/sia-rs/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)

## ⚠️ Status: work in progress, not a certified implementation

This crate is **not** a complete, audited, or certified DC-09 implementation.
It was built incrementally while learning the protocol from the official
SIA DC-09-2021 standard, supplemented by publicly available materials — a
public review draft of the underlying DC-07 protocol, and independent
technical write-ups and event-code references. It has **not** been reviewed
against the full paid SIA specification set (DC-03/04/05), nor tested
against a wide range of real central station receivers — only against one
test server.

Concretely, as of now:

- Only the `SIA-DCS` message format is supported (not `ADM-CID`).
- No encryption (AES) support — messages are sent unencrypted only.
- Only the account number is format-validated; prefix and receiver are not.
- No supervision/`NULL` message support.
- No retry-with-same-sequence-number handling (needed for NAK retries and
  redundant reporting paths).
- Response decoding is intentionally minimal: it classifies ACK / DUH / NAK,
  without verifying the CRC of the response itself or extracting anything
  beyond the response code.

If you're evaluating this for a real alarm deployment, read the source and
verify it against the standard yourself — don't take working test output as
proof of full conformance.

## What it does

- Builds correctly framed, CRC-checked `SIA-DCS` event messages (`Client`,
  `Account`).
- Tracks and increments the account's sequence number, wrapping `9999` back
  to `0001` as required by the standard.
- Validates the account number's format before constructing an `Account`.
- Classifies a server's response as ACK, DUH, or NAK (`check_response`).

## What it deliberately does not do

This crate does no networking. It builds message bytes and decodes response
bytes you already have — opening a socket, sending, and reading is up to
you. See `src/main.rs` for a minimal example of wiring it up over TCP.

## Installation

```toml
[dependencies]
sia-rs = { git = "https://github.com/lepetoc/sia-rs" }
```

*(Not published to crates.io yet.)*

## Usage

```rust
use sia_rs::{check_response, Account, Client};

let account = Account::new("56789", "0", None)?;
let mut client = Client::new(account);
let message = client.build_event("NFA0001");

// send `message` over your own TCP/UDP connection, read the response,
// then:
match check_response(&response_bytes) {
    Ok(()) => println!("acknowledged"),
    Err(e) => eprintln!("not accepted: {e}"),
}
```

## Sources

- SIA DC-09-2021, *Internet Protocol Event Reporting*
- SIA DC-07-2012 (public review draft), *Receiver-to-Computer Interface Protocol*
- Third-party SIA event code references (Chipkin Automation Systems, and others)
- [pysiaalarm](https://github.com/eavanvalkenburg/pysiaalarm), used as a
  reference implementation for cross-checking behavior (no code reused)

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE)
at your option.

## License

This project is licensed under either of

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](https://opensource.org/license/MIT)

at your option.
