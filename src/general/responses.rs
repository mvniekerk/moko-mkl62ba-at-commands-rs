use atat::serde_at::HexStr;
use atat::Error as AtatError;
use atat_derive::AtatResp;
use defmt::Format;
use heapless::String;

/// OK response
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct OkResponse {
    pub ok: String<4>,
}

impl OkResponse {
    pub fn is_ok(&self) -> bool {
        self.ok.as_str().eq("OK")
    }
}

/// ON/OFF responses
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct OnOff {
    pub on_off: String<6>,
}

impl OnOff {
    pub fn is_on(&self) -> bool {
        self.on_off.as_str().eq("ON")
    }
    pub fn is_off(&self) -> bool {
        self.on_off.as_str().eq("OFF")
    }
}

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct ErrorResponse {
    pub error: String<20>,
}

pub enum Error {
    /// ERROR (-1)
    AtCommandError,

    /// ERROR (-2)
    AtParameterError,

    /// ERROR (-3)
    Busy,

    /// ERROR (-5)
    CouldNotJoinTheNetwork,

    /// ERROR (-7)
    Timeout,
    Unknown,
}

impl Into<Error> for ErrorResponse {
    fn into(self) -> Error {
        match self.error.as_str() {
            "ERROR (-1)" => Error::AtCommandError,
            "ERROR (-2)" => Error::AtParameterError,
            "ERROR (-3)" => Error::Busy,
            "ERROR (-5)" => Error::CouldNotJoinTheNetwork,
            "ERROR (-7)" => Error::Timeout,
            _ => Error::Unknown,
        }
    }
}

impl Into<AtatError> for ErrorResponse {
    fn into(self) -> AtatError {
        match self.error.as_str() {
            "ERROR (-1)" => AtatError::Error,
            "ERROR (-2)" => AtatError::Parse,
            "ERROR (-3)" => AtatError::Overflow,
            "ERROR (-5)" => AtatError::Error,
            "ERROR (-7)" => AtatError::Timeout,
            _ => AtatError::Error,
        }
    }
}

impl From<AtatError> for Error {
    fn from(value: AtatError) -> Self {
        match value {
            AtatError::Read => Self::AtCommandError,
            AtatError::Write => Self::AtCommandError,
            AtatError::Timeout => Self::Timeout,
            AtatError::InvalidResponse => Self::AtCommandError,
            AtatError::Aborted => Self::AtCommandError,
            AtatError::Overflow => Self::Busy,
            AtatError::Parse => Self::AtParameterError,
            AtatError::Error => Self::AtParameterError,
            AtatError::CmeError(_) => Self::Unknown,
            AtatError::CmsError(_) => Self::Unknown,
            AtatError::ConnectionError(_) => Self::CouldNotJoinTheNetwork,
            AtatError::Custom => Self::Unknown,
        }
    }
}

impl Format for Error {
    fn format(&self, f: defmt::Formatter) {
        match self {
            Error::AtCommandError => defmt::write!(f, "AtCommandError"),
            Error::AtParameterError => defmt::write!(f, "AtParameterError"),
            Error::Busy => defmt::write!(f, "Busy"),
            Error::CouldNotJoinTheNetwork => defmt::write!(f, "CouldNotJoinTheNetwork"),
            Error::Timeout => defmt::write!(f, "Timeout"),
            Error::Unknown => defmt::write!(f, "Unknown"),
        }
    }
}

/// Get AppEUI response
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct AppEui {
    #[at_arg(position = 0)]
    pub app_eui: HexStr<u64>,
}

#[cfg(test)]
mod tests {
    use crate::general::responses::{OkResponse, OnOff};

    #[test]
    fn verify_ok() {
        let v = OkResponse {
            ok: heapless::String::from("OK"),
        };
        assert!(v.is_ok())
    }

    #[test]
    fn verify_on_off() {
        let k = OnOff {
            on_off: heapless::String::from("ON"),
        };
        assert!(k.is_on());
        let k = OnOff {
            on_off: heapless::String::from("OFF"),
        };
        assert!(k.is_off());
    }
}
