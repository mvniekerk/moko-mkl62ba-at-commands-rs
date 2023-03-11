use atat::serde_at::HexStr;
use atat_derive::AtatCmd;
use heapless::String;

pub mod responses;
pub mod types;

use responses::{
    AppEuiGet as AppEuiGetVal, AppKeyGet as AppKeyGetVal, DevEuiGet as DevEuiGetVal, LoraJoinMode,
    LoraRegionGet as LoraRegionGetVal, LoraClassGet as LoraClassGetVal
};

use types::{LoraRegion, LoraClass};

/// 4.3.1 Get Lora Join Mode
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+JOIN_MODE=?", LoraJoinMode)]
pub struct JoinModeGet {}

/// 4.3.1 Set Lora Join Mode
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+JOIN_MODE", LoraJoinMode, quote_escape_strings = false)]
pub struct JoinModeSet {
    pub join_mode: String<8>,
}

impl JoinModeSet {
    pub fn otaa() -> Self {
        Self {
            join_mode: String::from("OTAA"),
        }
    }

    pub fn abp() -> Self {
        Self {
            join_mode: String::from("ABP"),
        }
    }
}

/// 4.3.2 Get DevEui
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+DEVEUI=?", DevEuiGetVal)]
pub struct DevEuiGet {}

/// 4.3.2 Set DevEui
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+DEVEUI", DevEuiGetVal, quote_escape_strings = false)]
pub struct DevEuiSet {
    pub dev_eui: HexStr<u64>,
}

impl DevEuiSet {
    pub fn dev_eui(val: u64) -> Self {
        Self {
            dev_eui: HexStr {
                val,
                add_0x_with_encoding: false,
                hex_in_caps: true,
                delimiter_after_nibble_count: 2,
                delimiter: ':',
            },
        }
    }
}

/// 4.3.3 Get AppEui
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+APPEUI=?", AppEuiGetVal)]
pub struct AppEuiGet {}

/// 4.3.3 Set AppEui
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+APPEUI", AppEuiGetVal, quote_escape_strings = false)]
pub struct AppEuiSet {
    pub app_eui: HexStr<u64>,
}

impl AppEuiSet {
    pub fn app_eui(val: u64) -> Self {
        Self {
            app_eui: HexStr {
                val,
                add_0x_with_encoding: false,
                hex_in_caps: true,
                delimiter_after_nibble_count: 2,
                delimiter: ':',
            },
        }
    }
}

/// 4.3.4 Get AppKey
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+APPKEY=?", AppKeyGetVal)]
pub struct AppKeyGet {}

/// 4.3.4 Set AppKey
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+APPKEY", AppKeyGetVal, quote_escape_strings = false)]
pub struct AppKeySet {
    pub app_key: HexStr<u128>,
}

impl AppKeySet {
    pub fn app_key(val: u128) -> Self {
        Self {
            app_key: HexStr {
                val,
                add_0x_with_encoding: false,
                hex_in_caps: true,
                delimiter_after_nibble_count: 2,
                delimiter: ':',
            },
        }
    }
}

/// 4.3.8 Region Get
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+REGION=?", LoraRegionGetVal)]
pub struct LoraRegionGet {}

/// 4.3.8 Region Set
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+REGION", LoraRegionGetVal, quote_escape_strings = false)]
pub struct LoraRegionSet {
    pub region: String<10>
}

impl LoraRegionSet {
    pub fn region(region: LoraRegion) -> Self {
        region.set_cmd()
    }
}

/// 4.3.9 Class Get
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+CLASS=?", LoraClassGetVal)]
pub struct LoraClassGet {}

/// 4.3.9 Class Set
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+CLASS", LoraClassGetVal, quote_escape_strings = false)]
pub struct LoraClassSet {
    pub class: String<2>
}

impl LoraClassSet {
    pub fn class(class: LoraClass) -> Self {
        class.set_cmd()
    }
}



#[cfg(test)]
mod tests {
    use crate::lora::{AppEuiGet, AppEuiSet, AppKeyGet, AppKeySet, DevEuiGet, DevEuiSet, JoinModeGet, JoinModeSet, LoraClassGet, LoraRegionGet};
    use atat::AtatCmd;
    use crate::lora::types::{LoraClass, LoraRegion};

    #[test]
    fn join_mode_get() {
        let k = JoinModeGet {}.as_bytes();
        assert_eq!(k, b"AT+JOIN_MODE=?\r\n");
    }

    #[test]
    fn join_mode_set() {
        let k = JoinModeSet::otaa().as_bytes();
        assert_eq!(k, b"AT+JOIN_MODE=OTAA\r\n");
        let k = JoinModeSet::abp().as_bytes();
        assert_eq!(k, b"AT+JOIN_MODE=ABP\r\n");
    }

    #[test]
    fn dev_eui_get() {
        let k = DevEuiGet {}.as_bytes();
        assert_eq!(k, b"AT+DEVEUI=?\r\n");
    }

    #[test]
    fn dev_eui_set() {
        let k = DevEuiSet::dev_eui(0x303235386B375F03).as_bytes();
        assert_eq!(k, b"AT+DEVEUI=30:32:35:38:6B:37:5F:03\r\n");
    }

    #[test]
    fn app_eui_get() {
        let k = AppEuiGet {}.as_bytes();
        assert_eq!(k, b"AT+APPEUI=?\r\n");
    }

    #[test]
    fn app_eui_set() {
        let k = AppEuiSet::app_eui(0x303235386B375F03).as_bytes();
        assert_eq!(k, b"AT+APPEUI=30:32:35:38:6B:37:5F:03\r\n");
    }

    #[test]
    fn app_key_get() {
        let k = AppKeyGet {}.as_bytes();
        assert_eq!(k, b"AT+APPKEY=?\r\n");
    }

    #[test]
    fn app_key_set() {
        let k = AppKeySet::app_key(0x303235386B375F03_303235386B375F03).as_bytes();
        assert_eq!(
            k,
            b"AT+APPKEY=30:32:35:38:6B:37:5F:03:30:32:35:38:6B:37:5F:03\r\n"
        );
    }

    #[test]
    fn lora_region_get() {
        let k = LoraRegionGet {}.as_bytes();
        assert_eq!(k, b"AT+REGION=?\r\n");
    }

    #[test]
    fn lora_region_set() {
        let k = LoraRegion::Eu868.set_cmd().as_bytes();
        assert_eq!(k, b"AT+REGION=EU868\r\n");
    }

    #[test]
    fn lora_class_get() {
        let k = LoraClassGet {}.as_bytes();
        assert_eq!(k, b"AT+CLASS=?\r\n");
    }

    #[test]
    fn lora_class_set() {
        let k = LoraClass::ClassC.set_cmd().as_bytes();
        assert_eq!(k, b"AT+CLASS=C\r\n");
    }
}
