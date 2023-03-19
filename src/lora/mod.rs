pub mod commands;
pub mod responses;
pub mod types;

// use crate::client::MokoMkl62BaClient;
// use crate::general::responses::Error;
// use crate::lora::responses::LoraSendBytesResponse;
// use crate::lora::types::{LoraClass, LoraJoinMode, LoraJoiningStatus, LoraRegion};
// use atat::clock::Clock;
//
// impl<C, CLK, const TIMER_HZ: u32> MokoMkl62BaClient<C, CLK, TIMER_HZ>
// where
//     C: atat::AtatClient,
//     CLK: Clock<TIMER_HZ>,
// {
//     pub fn join_mode(&mut self) -> Result<LoraJoinMode, Error> {
//         let command = commands::JoinModeGet {};
//         let response = self.send_internal(&command, true)?;
//         Ok(response.mode())
//     }
//
//     pub fn join_mode_set(&mut self, mode: LoraJoinMode) -> Result<LoraJoinMode, Error> {
//         let command = match mode {
//             LoraJoinMode::Otaa => commands::JoinModeSet::otaa(),
//             LoraJoinMode::Abp => commands::JoinModeSet::abp(),
//             _ => return Err(Error::AtParameterError),
//         };
//         let response = self.send_internal(&command, true)?;
//         Ok(response.mode())
//     }
//
//     pub fn dev_eui(&mut self) -> Result<u64, Error> {
//         let command = commands::DevEuiGet {};
//         let response = self.send_internal(&command, true)?;
//         Ok(response.dev_eui.val)
//     }
//
//     pub fn dev_eui_set(&mut self, dev_eui: u64) -> Result<u64, Error> {
//         let command = commands::DevEuiSet::dev_eui(dev_eui);
//         let response = self.send_internal(&command, true)?;
//         Ok(response.dev_eui.val)
//     }
//
//     pub fn app_eui(&mut self) -> Result<u64, Error> {
//         let command = commands::AppEuiGet {};
//         let response = self.send_internal(&command, true)?;
//         Ok(response.app_eui.val)
//     }
//
//     pub fn app_eui_set(&mut self, app_eui: u64) -> Result<u64, Error> {
//         let command = commands::AppEuiSet::app_eui(app_eui);
//         let response = self.send_internal(&command, true)?;
//         Ok(response.app_eui.val)
//     }
//
//     pub fn app_key(&mut self) -> Result<u128, Error> {
//         let command = commands::AppKeyGet {};
//         let response = self.send_internal(&command, true)?;
//         Ok(response.app_key.val)
//     }
//
//     pub fn app_key_set(&mut self, app_key: u128) -> Result<u128, Error> {
//         let command = commands::AppKeySet::app_key(app_key);
//         let response = self.send_internal(&command, true)?;
//         Ok(response.app_key.val)
//     }
//
//     pub fn lora_region(&mut self) -> Result<LoraRegion, Error> {
//         let command = commands::LoraRegionGet {};
//         let response = self.send_internal(&command, true)?;
//         Ok(response.into())
//     }
//
//     pub fn lora_region_set(&mut self, region: LoraRegion) -> Result<LoraRegion, Error> {
//         let command = commands::LoraRegionSet::region(region);
//         let response = self.send_internal(&command, true)?;
//         Ok(response.into())
//     }
//
//     pub fn lora_class(&mut self) -> Result<LoraClass, Error> {
//         let command = commands::LoraClassGet {};
//         let response = self.send_internal(&command, true)?;
//         Ok(response.into())
//     }
//
//     pub fn lora_class_set(&mut self, class: LoraClass) -> Result<LoraClass, Error> {
//         let command = commands::LoraClassSet::class(class);
//         let response = self.send_internal(&command, true)?;
//         Ok(response.into())
//     }
//
//     pub fn lora_join_otaa(&mut self) -> Result<LoraJoiningStatus, Error> {
//         let command = commands::LoraJoinOtaa {};
//         let response = self.send_internal(&command, true)?;
//         Ok(response.into())
//     }
//
//     pub fn lora_join_status(&mut self) -> Result<LoraJoiningStatus, Error> {
//         let command = commands::LoraJoinOtaaStatus {};
//         let response = self.send_internal(&command, true)?;
//         Ok(response.into())
//     }
//
//     pub fn auto_join(&mut self) -> Result<bool, Error> {
//         let command = commands::LoraAutoJoinGet {};
//         let response = self.send_internal(&command, true)?;
//         Ok(response.is_on())
//     }
//
//     pub fn auto_join_set(&mut self, is_on: bool) -> Result<bool, Error> {
//         let command = if is_on {
//             commands::LoraAutoJoinSet::on()
//         } else {
//             commands::LoraAutoJoinSet::off()
//         };
//         let response = self.send_internal(&command, true)?;
//         Ok(response.is_on())
//     }
//
//     pub fn max_tx_len(&mut self) -> Result<u16, Error> {
//         let command = commands::LoraMaxTxLengthGet {};
//         let response = self.send_internal(&command, true)?;
//         Ok(response.max)
//     }
//
//     pub fn confirm_send(&mut self) -> Result<bool, Error> {
//         let command = commands::UplinkConfirmGet {};
//         let response = self.send_internal(&command, true)?;
//         Ok(response.is_on())
//     }
//
//     pub fn confirm_send_set(&mut self, is_on: bool) -> Result<bool, Error> {
//         let command = if is_on {
//             commands::UplinkConfirmSet::on()
//         } else {
//             commands::UplinkConfirmSet::off()
//         };
//         let response = self.send_internal(&command, true)?;
//         Ok(response.is_on())
//     }
//
//     pub fn send(
//         &mut self,
//         retransmission_times: u8,
//         port: u8,
//         data: &[u8],
//     ) -> Result<LoraSendBytesResponse, Error> {
//         let command = commands::SendBytes::new(retransmission_times, port, data);
//         let response = self.send_internal(&command, true)?;
//         Ok(response.into())
//     }
// }
