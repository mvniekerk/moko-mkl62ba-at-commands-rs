use atat::serde_at::HexStr;
use atat_derive::AtatResp;
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
