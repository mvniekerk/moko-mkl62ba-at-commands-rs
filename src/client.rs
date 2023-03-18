use defmt::{debug, error};
use crate::general::responses::Error;
use atat::clock::Clock;

pub struct MokoMkl62BaClient<C, CLK, const TIMER_HZ: u32>
where
    C: atat::AtatClient,
    CLK: Clock<TIMER_HZ>,
{
    pub(crate) inited: bool,
    pub(crate) client: C,
    pub(crate) timer: CLK,
}

impl<C, CLK, const TIMER_HZ: u32> MokoMkl62BaClient<C, CLK, TIMER_HZ>
where
    C: atat::AtatClient,
    CLK: Clock<TIMER_HZ>,
{
    pub fn new(client: C, timer: CLK) -> Self {
        Self {
            inited: false,
            client,
            timer,
        }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        debug!("Init Moko MKL62BA");
        Ok(())
    }

    pub(crate) fn send_internal<A, const LEN: usize>(
        &mut self,
        req: &A,
        check_urc: bool,
    ) -> Result<A::Response, Error>
    where
        A: atat::AtatCmd<LEN>,
    {
        if check_urc {
            if let Err(e) = self.handle_urc() {
                error!("Failed handle URC: {:?}", e);
            }
        }
        self.client.send(req).map_err(|e| match e {
            nb::Error::Other(ate) => {
                error!("{=[u8]:a}", req.as_bytes());
                ate.into()
            }
            nb::Error::WouldBlock => Error::Unknown,
        })
    }

    // TODO error
    fn handle_urc(&mut self) -> Result<bool, Error> {
        Ok(true)
    }


}
