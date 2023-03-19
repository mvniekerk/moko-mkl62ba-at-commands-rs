//! # URC parser implementation
//!
//! This is just used internally, but needs to be public for passing [URCMessages] as a generic to
//! [AtDigester](atat::digest::AtDigester): `AtDigester<URCMessages>`.

use atat::digest::ParseError;
use atat::{AtatUrc, Parser};

/// URC definitions, needs to passed as generic of [AtDigester](atat::digest::AtDigester): `AtDigester<URCMessages>`
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum URCMessages<const RX_SIZE: usize> {
    /// Unknown URC message
    Unknown,
}

impl<const RX_SIZE: usize> AtatUrc for URCMessages<RX_SIZE> {
    type Response = Self;

    fn parse(_resp: &[u8]) -> Option<Self::Response> {
        // Command echo
        Some(Self::Unknown)
    }
}

impl<const RX_SIZE: usize> Parser for URCMessages<RX_SIZE> {
    fn parse(_buf: &[u8]) -> Result<(&[u8], usize), ParseError> {
        Err(ParseError::Incomplete)
    }
}
