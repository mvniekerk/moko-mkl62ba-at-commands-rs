use super::types::{LoraClass, LoraJoinMode as LoraJoinModeVal, LoraRegion as LoraRegionVal};
use crate::lora::types::LoraJoiningStatus;
use atat::serde_at::HexStr;
use atat_derive::AtatResp;
use heapless::String;
use heapless_bytes::Bytes;
/// Lora Join Mode
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct LoraJoinMode {
    pub join_mode: String<8>,
}

impl LoraJoinMode {
    pub fn mode(&self) -> LoraJoinModeVal {
        match self.join_mode.as_str() {
            "OTAA" => LoraJoinModeVal::Otaa,
            "ABP" => LoraJoinModeVal::Abp,
            _ => LoraJoinModeVal::_Unknown,
        }
    }
}

/// DevEui Get
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct DevEuiGet {
    pub dev_eui: HexStr<u64>,
}

/// AppEui Get
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct AppEuiGet {
    pub app_eui: HexStr<u64>,
}

/// AppKey Get
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct AppKeyGet {
    pub app_key: HexStr<u128>,
}

/// Region
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct LoraRegionGet {
    pub region: String<10>,
}

impl From<LoraRegionGet> for LoraRegionVal {
    fn from(value: LoraRegionGet) -> Self {
        value.region.into()
    }
}

/// Class get
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct LoraClassGet {
    pub class: String<2>,
}

impl From<LoraClassGet> for LoraClass {
    fn from(value: LoraClassGet) -> Self {
        value.class.into()
    }
}

/// Joining
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct LoraJoinResponse {
    pub joining: String<20>,
}

impl From<LoraJoinResponse> for LoraJoiningStatus {
    fn from(value: LoraJoinResponse) -> Self {
        value.joining.into()
    }
}

/// Max TX length
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct LoraMaxTxLength {
    pub max: u16,
}

/// Send bytes response, unprocessed. Needs to change : to , in order for AtAt to work
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct LoraSendBytesResponseUnprocessed {
    pub val: Bytes<1044>,
}

/// Parsed send bytes response
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct LoraSendBytesResponse {
    pub retransmission_times: u8,
    pub port: u8,
    pub data: HexStr<[u8; 256]>,
}

impl From<LoraSendBytesResponseUnprocessed> for LoraSendBytesResponse {
    fn from(value: LoraSendBytesResponseUnprocessed) -> Self {
        let mut val = value.val;
        for b in val.iter_mut() {
            if *b == b':' {
                *b = b',';
            }
        }
        let val = core::str::from_utf8(val.as_slice()).unwrap();
        serde_at::from_str(val).unwrap()
    }
}

/// Received bytes response, raw.
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct LoraReceivedBytesResponseRaw {
    pub value: Bytes<1060>,
}

/// Parsed send bytes response
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct LoraReceivedBytesAckResponse {
    pub rssi: i32,
    pub snr: f32,
    pub ack: String<6>,
}

/// Parsed send bytes response
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct LoraReceivedBytesDataResponse {
    pub rssi: i32,
    pub snr: f32,
    pub port: u8,
    pub length: u16,
    pub data: HexStr<[u8; 256]>,
}

/// Data rate (DR) set response
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct DrSetResponse {
    pub data_rate: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoraReceivedBytes {
    None,
    Ack(LoraReceivedBytesAckResponse),
    Data(LoraReceivedBytesDataResponse),
}

impl From<LoraReceivedBytesResponseRaw> for LoraReceivedBytes {
    fn from(value: LoraReceivedBytesResponseRaw) -> Self {
        if value.value.is_empty() {
            Self::None
        } else {
            let mut val = value.value;
            for b in val.iter_mut() {
                if *b == b':' {
                    *b = b',';
                }
            }
            let val = core::str::from_utf8(val.as_slice()).unwrap();
            if val.ends_with(",ACK") {
                Self::Ack(serde_at::from_str(val).unwrap())
            } else {
                Self::Data(serde_at::from_str(val).unwrap())
            }
        }
    }
}

impl LoraReceivedBytesResponseRaw {
    pub fn processed(self) -> LoraReceivedBytes {
        self.into()
    }
}

/// Uplink frame count response
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct UplinkFrameCountResponse {
    pub uplink_frame_count: u32,
}

/// Downlink frame count response
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct DownlinkFrameCountResponse {
    pub downlink_frame_count: u32,
}

#[cfg(test)]
mod tests {
    use crate::lora::responses::{
        LoraClassGet, LoraReceivedBytes, LoraReceivedBytesAckResponse,
        LoraReceivedBytesDataResponse, LoraReceivedBytesResponseRaw, LoraRegionGet,
        LoraSendBytesResponse, LoraSendBytesResponseUnprocessed,
    };
    use crate::lora::types::{LoraClass, LoraRegion as LoraRegionVal};
    use core::str::FromStr;
    use heapless::String;
    use heapless_bytes::Bytes;
    use serde_at::HexStr;

    #[test]
    fn lora_region() {
        let r = LoraRegionGet {
            region: String::from("EU868"),
        };
        let r: LoraRegionVal = r.into();
        assert_eq!(r, LoraRegionVal::Eu868);
    }

    #[test]
    fn lora_class() {
        let r = LoraClassGet { class: "C".into() };
        let r: LoraClass = r.into();
        assert_eq!(r, LoraClass::ClassC)
    }

    #[test]
    fn lora_send_bytes_response() {
        let r = LoraSendBytesResponseUnprocessed {
            val: Bytes::from_slice(b"3:12:ABCDEF").unwrap(),
        };
        let r: LoraSendBytesResponse = r.into();
        assert_eq!(r.retransmission_times, 3);
        assert_eq!(r.port, 12);
        let mut v = [0; 256];
        v[0] = 0xAB;
        v[1] = 0xCD;
        v[2] = 0xEF;
        assert_eq!(*r.data, v);
    }

    #[test]
    fn received_bytes() {
        let value = Bytes::from_slice(b"").unwrap();
        let k = LoraReceivedBytesResponseRaw { value }.processed();
        assert_eq!(k, LoraReceivedBytes::None);

        let value = Bytes::from_slice(b"-104:1:ACK").unwrap();
        let k = LoraReceivedBytesResponseRaw { value }.processed();
        assert_eq!(
            k,
            LoraReceivedBytes::Ack(LoraReceivedBytesAckResponse {
                rssi: -104,
                snr: 1.0,
                ack: "ACK".into(),
            })
        );

        let value = Bytes::from_slice(b"-102:-4.0:8:8:3132333435363738").unwrap();
        let k = LoraReceivedBytesResponseRaw { value }.processed();
        let mut data = [0; 256];
        data[0] = 0x31;
        data[1] = 0x32;
        data[2] = 0x33;
        data[3] = 0x34;
        data[4] = 0x35;
        data[5] = 0x36;
        data[6] = 0x37;
        data[7] = 0x38;
        let data = HexStr {
            val: data,
            add_0x_with_encoding: false,
            hex_in_caps: true,
            delimiter_after_nibble_count: 0,
            delimiter: ' ',
            skip_last_0_values: false,
        };
        assert_eq!(
            k,
            LoraReceivedBytes::Data(LoraReceivedBytesDataResponse {
                rssi: -102,
                snr: -4.0,
                port: 8,
                length: 8,
                data,
            })
        );
    }
}
