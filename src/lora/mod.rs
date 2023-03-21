pub mod commands;
pub mod responses;
pub mod types;

#[cfg(feature = "async")]
pub mod asynch {
    use crate::client::asynch::MokoMkl62BaClient;
    use crate::lora::{
        commands,
        responses::LoraSendBytesResponse,
        types::{LoraClass, LoraJoinMode, LoraJoiningStatus, LoraRegion},
    };
    use atat::asynch::AtatClient;
    use atat::Error;
    use embedded_io::asynch::Write;

    impl<'a, W: Write, const INGRESS_BUF_SIZE: usize> MokoMkl62BaClient<'a, W, INGRESS_BUF_SIZE> {
        pub async fn join_mode(&mut self) -> Result<LoraJoinMode, Error> {
            let command = commands::JoinModeGet {};
            let response = self.client.send(&command).await?;
            Ok(response.mode())
        }

        pub async fn join_mode_set(&mut self, mode: LoraJoinMode) -> Result<LoraJoinMode, Error> {
            let command = match mode {
                LoraJoinMode::Otaa => commands::JoinModeSet::otaa(),
                LoraJoinMode::Abp => commands::JoinModeSet::abp(),
                _ => return Err(Error::Error),
            };
            let response = self.client.send(&command).await?;
            Ok(response.mode())
        }

        pub async fn dev_eui(&mut self) -> Result<u64, Error> {
            let command = commands::DevEuiGet {};
            let response = self.client.send(&command).await?;
            Ok(response.dev_eui.val)
        }

        pub async fn dev_eui_set(&mut self, dev_eui: u64) -> Result<u64, Error> {
            let command = commands::DevEuiSet::dev_eui(dev_eui);
            let response = self.client.send(&command).await?;
            Ok(response.dev_eui.val)
        }

        pub async fn app_eui(&mut self) -> Result<u64, Error> {
            let command = commands::AppEuiGet {};
            let response = self.client.send(&command).await?;
            Ok(response.app_eui.val)
        }

        pub async fn app_eui_set(&mut self, app_eui: u64) -> Result<u64, Error> {
            let command = commands::AppEuiSet::app_eui(app_eui);
            let response = self.client.send(&command).await?;
            Ok(response.app_eui.val)
        }

        pub async fn app_key(&mut self) -> Result<u128, Error> {
            let command = commands::AppKeyGet {};
            let response = self.client.send(&command).await?;
            Ok(response.app_key.val)
        }

        pub async fn app_key_set(&mut self, app_key: u128) -> Result<u128, Error> {
            let command = commands::AppKeySet::app_key(app_key);
            let response = self.client.send(&command).await?;
            Ok(response.app_key.val)
        }

        pub async fn lora_region(&mut self) -> Result<LoraRegion, Error> {
            let command = commands::LoraRegionGet {};
            let response = self.client.send(&command).await?;
            Ok(response.into())
        }

        pub async fn lora_region_set(&mut self, region: LoraRegion) -> Result<LoraRegion, Error> {
            let command = commands::LoraRegionSet::region(region);
            let response = self.client.send(&command).await?;
            Ok(response.into())
        }

        pub async fn lora_class(&mut self) -> Result<LoraClass, Error> {
            let command = commands::LoraClassGet {};
            let response = self.client.send(&command).await?;
            Ok(response.into())
        }

        pub async fn lora_class_set(&mut self, class: LoraClass) -> Result<LoraClass, Error> {
            let command = commands::LoraClassSet::class(class);
            let response = self.client.send(&command).await?;
            Ok(response.into())
        }

        pub async fn lora_join_otaa(&mut self) -> Result<LoraJoiningStatus, Error> {
            let command = commands::LoraJoinOtaa {};
            let response = self.client.send(&command).await?;
            Ok(response.into())
        }

        pub async fn lora_join_status(&mut self) -> Result<LoraJoiningStatus, Error> {
            let command = commands::LoraJoinOtaaStatus {};
            let response = self.client.send(&command).await?;
            Ok(response.into())
        }

        pub async fn auto_join(&mut self) -> Result<bool, Error> {
            let command = commands::LoraAutoJoinGet {};
            let response = self.client.send(&command).await?;
            Ok(response.is_on())
        }

        pub async fn auto_join_set(&mut self, is_on: bool) -> Result<bool, Error> {
            let command = if is_on {
                commands::LoraAutoJoinSet::on()
            } else {
                commands::LoraAutoJoinSet::off()
            };
            let response = self.client.send(&command).await?;
            Ok(response.is_on())
        }

        pub async fn max_tx_len(&mut self) -> Result<u16, Error> {
            let command = commands::LoraMaxTxLengthGet {};
            let response = self.client.send(&command).await?;
            Ok(response.max)
        }

        pub async fn confirm_send(&mut self) -> Result<bool, Error> {
            let command = commands::UplinkConfirmGet {};
            let response = self.client.send(&command).await?;
            Ok(response.is_on())
        }

        pub async fn confirm_send_set(&mut self, is_on: bool) -> Result<bool, Error> {
            let command = if is_on {
                commands::UplinkConfirmSet::on()
            } else {
                commands::UplinkConfirmSet::off()
            };
            let response = self.client.send(&command).await?;
            Ok(response.is_on())
        }

        pub async fn send(
            &mut self,
            retransmission_times: u8,
            port: u8,
            data: &[u8],
        ) -> Result<LoraSendBytesResponse, Error> {
            let command = commands::SendBytes::new(retransmission_times, port, data);
            let response = self.client.send(&command).await?;
            Ok(response.into())
        }

        pub async fn adr_set(&mut self, on: bool) -> Result<bool, Error> {
            let command = if on {
                commands::LoraAdrSet::on()
            } else {
                commands::LoraAdrSet::off()
            };
            let response = self.client.send(&command).await?;
            Ok(response.is_on())
        }

        pub async fn dr_set(&mut self, data_rate: u8) -> Result<u8, Error> {
            let command = commands::LoraDrSet { data_rate };
            let response = self.client.send(&command).await?;
            Ok(response.data_rate)
        }
    }
}
