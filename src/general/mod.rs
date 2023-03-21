pub mod commands;
pub mod responses;
pub mod types;

#[cfg(feature = "async")]
pub mod asynch {
    use crate::client::asynch::MokoMkl62BaClient;
    use crate::general::commands::{AteGet, AteSet, Reset, SleepGet, SleepSet, VerifyComIsWorking};
    use atat::asynch::AtatClient;
    use atat::Error;
    use embedded_io::asynch::Write;

    impl<'a, W: Write, const INGRESS_BUF_SIZE: usize> MokoMkl62BaClient<'a, W, INGRESS_BUF_SIZE> {
        pub async fn verify_com_is_working(&mut self) -> Result<bool, Error> {
            let command = VerifyComIsWorking {};
            let response = self.client.send(&command).await?;
            Ok(response.is_ok())
        }

        pub async fn at_echo_on(&mut self) -> Result<bool, Error> {
            let command = AteGet {};
            let response = self.client.send(&command).await?;
            Ok(response.is_on())
        }

        pub async fn at_echo_set(&mut self, on: bool) -> Result<bool, Error> {
            let command = if on { AteSet::on() } else { AteSet::off() };
            let response = self.client.send(&command).await?;
            Ok(response.is_on())
        }

        pub async fn sleep_status(&mut self) -> Result<bool, Error> {
            let command = SleepGet {};
            let response = self.client.send(&command).await?;
            Ok(response.is_on())
        }

        pub async fn sleep_set(&mut self, on: bool) -> Result<bool, Error> {
            let command = if on { SleepSet::on() } else { SleepSet::off() };
            let response = self.client.send(&command).await?;
            Ok(response.is_on())
        }

        pub async fn reset(&mut self) -> Result<(), Error> {
            let command = Reset {};
            self.client.send(&command).await?;
            Ok(())
        }
    }
}
