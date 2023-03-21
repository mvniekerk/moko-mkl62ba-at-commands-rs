use atat_derive::AtatCmd;
use heapless::String;

use super::responses::{OkResponse, OnOff};
use crate::{NoResponse};

/// 4.1.1 AT - Verify COM is working
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("AT", OkResponse, cmd_prefix = "", timeout_ms = 2000)]
pub struct VerifyComIsWorking {}

/// 4.1.3 Get ATE - Echo is on/off
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+ATE=?", OnOff)]
pub struct AteGet {}

/// 4.1.3 Set ATE - Echo is on/off
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+ATE", OnOff, quote_escape_strings = false, timeout_ms = 4000)]
pub struct AteSet {
    pub on: String<6>,
}

impl AteSet {
    pub fn on() -> Self {
        Self {
            on: String::from("ON"),
        }
    }
    pub fn off() -> Self {
        Self {
            on: String::from("OFF"),
        }
    }
}

/// 4.1.5 Get Sleep status
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+SLEEP=?", OnOff)]
pub struct SleepGet {}

/// 4.1.5 Set Sleep status
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+SLEEP", OnOff, quote_escape_strings = false, timeout_ms = 4000)]
pub struct SleepSet {
    pub on: String<6>,
}

impl SleepSet {
    pub fn on() -> Self {
        Self {
            on: String::from("ON"),
        }
    }
    pub fn off() -> Self {
        Self {
            on: String::from("OFF"),
        }
    }
}

///4.1.6 Reset
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+RESET", OkResponse, timeout_ms = 8000)]
pub struct Reset {}

#[cfg(test)]
mod tests {
    use crate::general::commands::{AteGet, AteSet, SleepSet, VerifyComIsWorking};
    use atat::AtatCmd;

    #[test]
    fn verify_com_is_working_serializes_correctly() {
        let k = VerifyComIsWorking {};
        let k = k.as_bytes();
        assert_eq!(k, b"AT\r\n");
    }

    #[test]
    fn ate_get() {
        let k = AteGet {}.as_bytes();
        assert_eq!(k, b"AT+ATE=?\r\n");
    }

    #[test]
    fn ate_set() {
        let k = AteSet::on().as_bytes();
        assert_eq!(k, b"AT+ATE=ON\r\n");
        let k = AteSet::off().as_bytes();
        assert_eq!(k, b"AT+ATE=OFF\r\n");
    }

    #[test]
    fn sleep_set() {
        let k = SleepSet::on().as_bytes();
        assert_eq!(k, b"AT+SLEEP=ON\r\n");
        let k = SleepSet::off().as_bytes();
        assert_eq!(k, b"AT+SLEEP=OFF\r\n");
    }
}
