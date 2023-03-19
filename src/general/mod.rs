pub mod commands;
pub mod responses;
pub mod types;

// use crate::client::MokoMkl62BaClient;
// use crate::general::commands::{AteGet, AteSet, SleepGet, SleepSet, VerifyComIsWorking};
// use crate::general::responses::Error;
// use atat::clock::Clock;
//
// impl<C, CLK, const TIMER_HZ: u32> MokoMkl62BaClient<C, CLK, TIMER_HZ>
// where
//     C: atat::AtatClient,
//     CLK: Clock<TIMER_HZ>,
// {
//     pub fn verify_com_is_working(&mut self) -> Result<bool, Error> {
//         let command = VerifyComIsWorking {};
//         let response = self.send_internal(&command, true)?;
//         Ok(response.is_ok())
//     }
//
//     pub fn at_echo_on(&mut self) -> Result<bool, Error> {
//         let command = AteGet {};
//         let response = self.send_internal(&command, true)?;
//         Ok(response.is_on())
//     }
//
//     pub fn at_echo_set(&mut self, on: bool) -> Result<bool, Error> {
//         let command = if on { AteSet::on() } else { AteSet::off() };
//         let response = self.send_internal(&command, true)?;
//         Ok(response.is_on())
//     }
//
//     pub fn sleep_status(&mut self) -> Result<bool, Error> {
//         let command = SleepGet {};
//         let response = self.send_internal(&command, true)?;
//         Ok(response.is_on())
//     }
//
//     pub fn sleep_set(&mut self, on: bool) -> Result<bool, Error> {
//         let command = if on { SleepSet::on() } else { SleepSet::off() };
//         let response = self.send_internal(&command, true)?;
//         Ok(response.is_on())
//     }
//
//     pub fn reset(&mut self) -> Result<(), Error> {
//         let command = crate::general::commands::Reset {};
//         self.send_internal(&command, true)?;
//         Ok(())
//     }
// }
