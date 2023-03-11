use super::types::{LoraJoinMode as LoraJoinModeVal, LoraRegion as LoraRegionVal, LoraClass};
use atat::serde_at::HexStr;
use atat_derive::AtatResp;
use heapless::String;

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

#[cfg(test)]
mod tests {
    use crate::lora::responses::{LoraClassGet, LoraRegionGet};
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
}
