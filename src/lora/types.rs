use heapless::String;
use crate::lora::{LoraClassSet, LoraRegionSet};

#[derive(Debug, Clone, PartialEq)]
pub enum LoraJoinMode {
    Otaa,
    Abp,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoraRegion {
    Eu868,
    In865,
    Ru864,
    Us915,
    Au915,
    As923,
    Unknown,
}

impl From<String<10>> for LoraRegion {
    fn from(value: String<10>) -> Self {
        match value.as_str() {
            "EU868" => Self::Eu868,
            "IN865" => Self::In865,
            "RU864" => Self::Ru864,
            "US915" => Self::Us915,
            "AU915" => Self::Au915,
            "AS923" => Self::As923,
            _ => Self::Unknown,
        }
    }
}

impl From<LoraRegion> for String<10> {
    fn from(value: LoraRegion) -> Self {
        match value {
            LoraRegion::Eu868 => "EU868".into(),
            LoraRegion::In865 => "IN865".into(),
            LoraRegion::Ru864 => "RU864".into(),
            LoraRegion::Us915 => "US915".into(),
            LoraRegion::Au915 => "AU915".into(),
            LoraRegion::As923 => "AS923".into(),
            LoraRegion::Unknown => "".into()
        }
    }
}

impl LoraRegion {
    pub fn set_cmd(self) -> LoraRegionSet {
        LoraRegionSet {
            region: self.into(),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum LoraClass {
    ClassA,
    ClassB,
    ClassC,
    Unknown
}

impl From<String<2>> for LoraClass {
    fn from(value: String<2>) -> Self {
        match value.as_str() {
            "A" => Self::ClassA,
            "B" => Self::ClassB,
            "C" => Self::ClassC,
            _ => Self::Unknown
        }
    }
}

impl From<LoraClass> for String<2> {
    fn from(value: LoraClass) -> Self {
        match value {
            LoraClass::ClassA => "A".into(),
            LoraClass::ClassB => "B".into(),
            LoraClass::ClassC => "C".into(),
            LoraClass::Unknown => "".into()
        }
    }
}

impl LoraClass {
    pub fn set_cmd(self) -> LoraClassSet {
        LoraClassSet {
            class: self.into()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoraJoiningStatus {
    Joining,
    Joined,
    JoinFailed,
    BusyError,
    InAbpModeError,
    Unknown
}

impl From<String<20>> for LoraJoiningStatus {
    fn from(value: String<20>) -> Self {
        match value.as_str() {
            "JOINING" => Self::Joining,
            "JOINED" => Self::Joined,
            "JOIN FAILED" => Self::JoinFailed,
            "ERROR(-3)" => Self::BusyError,
            "ERROR(-2)" => Self::InAbpModeError,
            _ => Self::Unknown
        }
    }
}