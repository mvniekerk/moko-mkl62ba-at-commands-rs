use super::types::{LoraJoinMode as LoraJoinModeVal, LoraRegion as LoraRegionVal, LoraClass};
use atat::serde_at::HexStr;
use atat_derive::AtatResp;
use heapless::String;
use crate::lora::types::LoraJoiningStatus;

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
            _ => LoraJoinModeVal::Unknown,
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
    pub joining: String<20>
}

impl From<LoraJoinResponse> for LoraJoiningStatus {
    fn from(value: LoraJoinResponse) -> Self {
        value.joining.into()
    }
}

/// Max TX length
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct LoraMaxTxLength {
    pub max: u16
}

/// Send bytes response, unprocessed. Needs to change : to , in order for AtAt to work
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct LoraSendBytesResponseUnprocessed {
    pub val: String<288>
}

/// Parsed send bytes response
#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct LoraSendBytesResponse {
    pub retransmission_times: u8,
    pub port: u8,
    pub data: HexStr<[u8; 256]>
}

impl From<LoraSendBytesResponseUnprocessed> for LoraSendBytesResponse {
    fn from(value: LoraSendBytesResponseUnprocessed) -> Self {
        let mut v: String<304> = String::new();
        let val = value.val;
        let val = val.replace(':', ",");
        v.push_str("+SENDB: ").unwrap();
        v.push_str(&val).unwrap();
        serde_at::from_str(v.as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::lora::responses::{LoraClassGet, LoraRegionGet, LoraSendBytesResponse, LoraSendBytesResponseUnprocessed};
    use crate::lora::types::{LoraClass, LoraRegion as LoraRegionVal};
    use heapless::String;

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
        let r = LoraClassGet {
            class: "C".into()
        };
        let r: LoraClass = r.into();
        assert_eq!(r, LoraClass::ClassC)
    }

    #[test]
    fn lora_send_bytes_response() {
        let r = LoraSendBytesResponseUnprocessed {
            val: "3:12:ABCDEF".into()
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
}
