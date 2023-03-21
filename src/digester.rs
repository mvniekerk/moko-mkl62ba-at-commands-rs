use atat::{nom, nom::{branch, bytes, character, combinator, sequence}, DigestResult, Digester, Parser};
use atat::{InternalError, digest::{parser, ParseError}};
use atat::helpers::LossyStr;
use defmt::{debug, info};
use crate::urc::URCMessages;

pub struct MokoDigester {}

impl MokoDigester {
    pub fn custom_error(buf: &[u8]) -> Result<(&[u8], usize), ParseError> {
        let (_reminder, (head, data, tail)) = branch::alt((
            sequence::tuple((
                combinator::success(&b""[..]),
                bytes::streaming::tag(b"ERROR(-1)"),
                bytes::streaming::tag(b"\r\n"),
            )),
            sequence::tuple((
                combinator::success(&b""[..]),
                bytes::streaming::tag(b"ERROR(-2)"),
                bytes::streaming::tag(b"\r\n"),
            )),
            sequence::tuple((
                combinator::success(&b""[..]),
                bytes::streaming::tag(b"ERROR(-3)"),
                bytes::streaming::tag(b"\r\n"),
            )),
            sequence::tuple((
                combinator::success(&b""[..]),
                bytes::streaming::tag(b"ERROR(-5)"),
                bytes::streaming::tag(b"\r\n"),
            )),
            sequence::tuple((
                combinator::success(&b""[..]),
                bytes::streaming::tag(b"ERROR(-7)"),
                bytes::streaming::tag(b"\r\n"),
            )),
        ))(buf)?;
        debug!("Custom error {:?}", LossyStr(data));
        Ok((data, head.len() + data.len() + tail.len()))
    }

    pub fn custom_success(buf: &[u8]) -> Result<(&[u8], usize), ParseError> {
        info!("Custom success start {:?}", LossyStr(buf));
        let (_reminder, (head, data, tail)) = branch::alt((
            // AT command
            sequence::tuple((
                bytes::streaming::tag(b"+AT: "),
                bytes::streaming::take_until("\r\n"),
                bytes::streaming::tag("\r\n"),
            )),
            // ATE
            sequence::tuple((
                combinator::success(&b""[..]),
                combinator::recognize(sequence::tuple((
                    bytes::streaming::tag(b"+ATE: "),
                    bytes::streaming::take_until("\r\n"),
                ))),
                branch::alt((
                    bytes::streaming::tag("\r\n"),
                    bytes::streaming::tag("OK\r\n"),
                )),
            )),
            // +RESET / Startup preamble
            sequence::tuple((
                combinator::success(&b""[..]),
                combinator::recognize(sequence::tuple((
                    bytes::streaming::tag(b"+ATZ: "),
                    bytes::streaming::take_while(character::is_alphanumeric),
                ))),
                combinator::recognize(sequence::tuple((
                    bytes::streaming::tag("\r\n"),
                    bytes::streaming::take_until("\r\n"),
                    bytes::streaming::tag("\r\n"),
                )))
            )),
            // Join status
            sequence::tuple((
                combinator::success(&b""[..]),
                combinator::recognize(sequence::tuple((
                    bytes::streaming::tag(b"+JOIN_STD: "),
                    bytes::streaming::take_until("\r\n"),
                ))),
                bytes::streaming::tag("\r\n"),
            )),

        ))(buf)?;
        info!("Custom success ! [{:?}]", LossyStr(data));
        Ok((data, head.len() + data.len() + tail.len()))
    }
}

impl Default for MokoDigester {
    fn default() -> Self {
        Self {}
    }
}

impl Digester for MokoDigester {
    fn digest<'a>(&mut self, input: &'a [u8]) -> (DigestResult<'a>, usize) {
        let s = LossyStr(input);
        info!("Digesting: {:?}", s);

        // Incomplete. Eat the echo and do nothing else.
        let incomplete = (DigestResult::None, 0);

        // Stray OK\r\n
        if input == b"OK\r\n" {
            return (DigestResult::None, 4);
        }

        // Generic success replies
        match parser::success_response(input) {
            Ok((_, (result, len))) => return (result, len),
            Err(nom::Err::Incomplete(_)) => return incomplete,
            _ => {}
        }

        // 2. Match for URC's
        match <URCMessages as Parser>::parse(input) {
            Ok((urc, len)) => return (DigestResult::Urc(urc), len),
            Err(ParseError::Incomplete) => return incomplete,
            _ => {}
        }

        // 3. Parse for success responses
        // Custom successful replies first, if any
        match (MokoDigester::custom_success)(input) {
            Ok((response, len)) => return (DigestResult::Response(Ok(response)), len),
            Err(ParseError::Incomplete) => return incomplete,
            _ => {}
        }

        // 4. Parse for error responses
        // Custom error matches first, if any
        match (MokoDigester::custom_error)(input) {
            Ok((response, len)) => {
                return (
                    DigestResult::Response(Err(InternalError::Custom(response))),
                    len,
                )
            }
            Err(ParseError::Incomplete) => return incomplete,
            _ => {}
        }

        // Generic error matches
        if let Ok((_, (result, len))) = parser::error_response(input) {
            return (result, len);
        }

        // No matches at all.
        incomplete
    }
}
