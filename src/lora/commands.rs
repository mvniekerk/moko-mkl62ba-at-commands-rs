use atat::serde_at::HexStr;
use atat_derive::AtatCmd;
use core::str::FromStr;
use heapless::String;
use serde_at::SerializeOptions;

use super::responses::{
    AppEuiGet as AppEuiGetVal, AppKeyGet as AppKeyGetVal, DevEuiGet as DevEuiGetVal,
    LoraClassGet as LoraClassGetVal, LoraJoinMode, LoraJoinResponse, LoraMaxTxLength,
    LoraReceivedBytesResponseRaw, LoraRegionGet as LoraRegionGetVal,
    LoraSendBytesResponseUnprocessed, DrSetResponse, UplinkFrameCountResponse, DownlinkFrameCountResponse
};

use super::types::{LoraClass, LoraRegion};

use crate::general::responses::OnOff;

/// 4.3.1 Get Lora Join Mode
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+JOIN_MODE=?", LoraJoinMode)]
pub struct JoinModeGet {}

/// 4.3.1 Set Lora Join Mode
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd(
    "+JOIN_MODE",
    LoraJoinMode,
    quote_escape_strings = false,
    timeout_ms = 4000
)]
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
#[at_cmd(
    "+DEVEUI",
    DevEuiGetVal,
    quote_escape_strings = false,
    timeout_ms = 4000
)]
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
                skip_last_0_values: false,
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
#[at_cmd(
    "+APPEUI",
    AppEuiGetVal,
    quote_escape_strings = false,
    timeout_ms = 4000
)]
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
                skip_last_0_values: false,
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
#[at_cmd(
    "+APPKEY",
    AppKeyGetVal,
    quote_escape_strings = false,
    timeout_ms = 4000
)]
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
                skip_last_0_values: false,
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
#[at_cmd(
    "+REGION",
    LoraRegionGetVal,
    quote_escape_strings = false,
    timeout_ms = 4000
)]
pub struct LoraRegionSet {
    pub region: String<10>,
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
#[at_cmd(
    "+CLASS",
    LoraClassGetVal,
    quote_escape_strings = false,
    timeout_ms = 4000
)]
pub struct LoraClassSet {
    pub class: String<2>,
}

impl LoraClassSet {
    pub fn class(class: LoraClass) -> Self {
        class.set_cmd()
    }
}

/// 4.3.10 Join using OTAA
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+JOINING", LoraJoinResponse, timeout_ms = 10000)]
pub struct LoraJoinOtaa {}

/// 4.3.11 Join status
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+JOIN_STD=?", LoraJoinResponse, timeout_ms = 4000)]
pub struct LoraJoinOtaaStatus {}

/// 4.3.12 Auto join get
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+AUTO_JOIN=?", OnOff, timeout_ms = 4000)]
pub struct LoraAutoJoinGet {}

/// 4.3.12 Auto join set
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+AUTO_JOIN", OnOff, quote_escape_strings = false, timeout_ms = 4000)]
pub struct LoraAutoJoinSet {
    pub on: String<6>,
}

impl LoraAutoJoinSet {
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

/// 4.3.15 ADR set
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+ADR", OnOff, quote_escape_strings = false, timeout_ms = 4000)]
pub struct LoraAdrSet {
    pub on: String<6>,
}

impl LoraAdrSet {
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

/// 4.3.17 Data rate (DR) set. 0 - 15
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+DR", DrSetResponse, quote_escape_strings = false, timeout_ms = 4000)]
pub struct LoraDrSet {
    pub data_rate: u8,
}

/// 4.4.1 Maximum TX length get
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+TX_LEN=?", LoraMaxTxLength, quote_escape_strings = false)]
pub struct LoraMaxTxLengthGet {}

/// 4.4.2 Uplink confirmation get
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+CONFIRM=?", OnOff)]
pub struct UplinkConfirmGet {}

/// 4.4.2 Uplink confirmation set
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+CONFIRM", OnOff, quote_escape_strings = false)]
pub struct UplinkConfirmSet {
    pub on: String<6>,
}

impl UplinkConfirmSet {
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

/// 4.4.3 Send bytes, unprocessed. The AT command sent is wrong and needs , replaced with :
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd(
    "+SENDB",
    LoraSendBytesResponseUnprocessed,
    quote_escape_strings = false
)]
pub struct SendBytesUnprocessed {
    pub retransmission_times: u8,
    pub port: u8,
    pub data: HexStr<[u8; 256]>,
}

/// 4.4.3 Send bytes, processed.
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd(
    "+SENDB",
    LoraSendBytesResponseUnprocessed,
    quote_escape_strings = false
)]
pub struct SendBytes {
    pub val: String<288>,
}

impl SendBytesUnprocessed {
    pub fn processed(self) -> SendBytes {
        let mut val: String<288> = serde_at::ser::to_string(
            &self,
            "",
            SerializeOptions {
                value_sep: false,
                cmd_prefix: "",
                termination: "",
                quote_escape_strings: false,
            },
        )
        .unwrap();
        unsafe {
            for b in String::as_mut_vec(&mut val) {
                if *b == b',' {
                    *b = b':';
                }
            }
        }
        SendBytes {
            val: String::from_str(&val).unwrap(),
        }
    }
}

impl SendBytes {
    pub fn new(retransmission_times: u8, port: u8, data: &[u8]) -> Self {
        let mut val = [0u8; 256];
        for (place, array) in val.iter_mut().zip(data.iter()) {
            *place = *array;
        }

        let data = HexStr {
            val,
            add_0x_with_encoding: false,
            hex_in_caps: true,
            delimiter_after_nibble_count: 0,
            delimiter: ' ',
            skip_last_0_values: true,
        };
        SendBytesUnprocessed {
            retransmission_times,
            port,
            data,
        }
        .processed()
    }
}

/// 4.4.5 Receive bytes or ACK from the server
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+RECVB=?", LoraReceivedBytesResponseRaw, quote_escape_strings = false)]
pub struct LoraReceiveBytes {}

/// 4.4.6 Uplink frame count
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+UP_CNT=?", UplinkFrameCountResponse)]
pub struct UplinkFrameCountGet {}

/// 4.4.7 Downlink frame count
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+DOWN_CNT=?", DownlinkFrameCountResponse)]
pub struct DownlinkFrameCountGet {}

#[cfg(test)]
mod tests {
    use crate::lora::commands::{
        AppEuiGet, AppEuiSet, AppKeyGet, AppKeySet, DevEuiGet, DevEuiSet, JoinModeGet, JoinModeSet,
        LoraAutoJoinGet, LoraAutoJoinSet, LoraClassGet, LoraJoinOtaa, LoraJoinOtaaStatus,
        LoraMaxTxLengthGet, LoraReceiveBytes, LoraRegionGet, SendBytes, SendBytesUnprocessed,
        UplinkConfirmGet, UplinkConfirmSet,
    };
    use crate::lora::types::{LoraClass, LoraRegion};
    use atat::AtatCmd;

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

    #[test]
    fn lora_join_otaa() {
        let k = LoraJoinOtaa {}.as_bytes();
        assert_eq!(k, b"AT+JOINING\r\n")
    }

    #[test]
    fn lora_join_otaa_status() {
        let k = LoraJoinOtaaStatus {}.as_bytes();
        assert_eq!(k, b"AT+JOIN_STD=?\r\n")
    }

    #[test]
    fn lora_auto_join_get() {
        let k = LoraAutoJoinGet {}.as_bytes();
        assert_eq!(k, b"AT+AUTO_JOIN=?\r\n");
    }

    #[test]
    fn lora_auto_join_set() {
        let k = LoraAutoJoinSet::on().as_bytes();
        assert_eq!(k, b"AT+AUTO_JOIN=ON\r\n");
        let k = LoraAutoJoinSet::off().as_bytes();
        assert_eq!(k, b"AT+AUTO_JOIN=OFF\r\n");
    }

    #[test]
    fn max_tx_len_get() {
        let k = LoraMaxTxLengthGet {}.as_bytes();
        assert_eq!(k, b"AT+TX_LEN=?\r\n");
    }

    #[test]
    fn uplink_confirm_get() {
        let k = UplinkConfirmGet {}.as_bytes();
        assert_eq!(k, b"AT+CONFIRM=?\r\n");
    }

    #[test]
    fn uplink_confirm_set() {
        let k = UplinkConfirmSet::on().as_bytes();
        assert_eq!(k, b"AT+CONFIRM=ON\r\n");
        let k = UplinkConfirmSet::off().as_bytes();
        assert_eq!(k, b"AT+CONFIRM=OFF\r\n");
    }

    #[test]
    fn send_bytes() {
        let mut v = [0; 256];
        v[0] = 0xAB;
        v[1] = 0xCD;
        v[2] = 0xEF;
        v[3] = 0x01;
        let k = SendBytes::new(3, 12, &v).as_bytes();
        assert_eq!(k, b"AT+SENDB=3:12:ABCDEF01\r\n");
    }

    #[test]
    fn receive_bytes() {
        let k = LoraReceiveBytes {}.as_bytes();
        assert_eq!(k, b"AT+RECVB=?\r\n");
    }
}
