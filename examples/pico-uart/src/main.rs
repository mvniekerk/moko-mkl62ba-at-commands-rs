#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::str::FromStr;
use defmt::{ info, unwrap};
use embassy_executor::Spawner;
use embassy_executor::_export::StaticCell;
use embassy_rp::interrupt;
use embassy_rp::peripherals::UART1;
use embassy_rp::uart::DataBits::DataBits8;
use embassy_rp::uart::{BufferedUart, BufferedUartRx, Config, Parity, StopBits};
use embedded_io::asynch::Read;
use {defmt_rtt as _, panic_probe as _};

use atat::bbqueue::BBBuffer;
use atat::{AtDigester, IngressManager};
use moko_mkl62ba_at_commands::client::MokoMkl62BaClient;
use moko_mkl62ba_at_commands::urc::URCMessages;


// Chunk size in bytes when sending data. Higher value results in better
// performance, but introduces also higher stack memory footprint. Max value: 8192.
const TX_SIZE: usize = 288;
// Chunk size in bytes when receiving data. Value should be matched to buffer
// size of receive() calls.
const RX_SIZE: usize = 288;

// Constants derived from TX_SIZE and RX_SIZE
const ESP_TX_SIZE: usize = TX_SIZE;
const ESP_RX_SIZE: usize = RX_SIZE;
const ATAT_RX_SIZE: usize = RX_SIZE;
const URC_RX_SIZE: usize = RX_SIZE;
const RES_CAPACITY: usize = RX_SIZE;
const URC_CAPACITY: usize = RX_SIZE * 3;

// Timer frequency in Hz
const TIMER_HZ: u32 = 1000;

type AtDigesterIngressManager = u32;
// type AtDigesterIngressManager =
//     IngressManager<AtDigester<URCMessages<URC_RX_SIZE>>, ATAT_RX_SIZE, RES_CAPACITY, URC_CAPACITY>;

macro_rules! singleton {
    ($val:expr) => {{
        type T = impl Sized;
        static STATIC_CELL: StaticCell<T> = StaticCell::new();
        let (x,) = STATIC_CELL.init(($val,));
        x
    }};
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let (tx_pin, rx_pin, uart) = (p.PIN_4, p.PIN_5, p.UART1);

    let irq = interrupt::take!(UART1_IRQ);
    let tx_buf = &mut singleton!([0u8; 32])[..];
    let rx_buf = &mut singleton!([0u8; 32])[..];
    let mut config = Config::default();
    config.baudrate = 9600;
    config.parity = Parity::ParityNone;
    config.stop_bits = StopBits::STOP1;
    config.data_bits = DataBits8;
    let mut uart = BufferedUart::new(uart, irq, tx_pin, rx_pin, tx_buf, rx_buf, config);
    let (rx, mut tx) = uart.split();

    static mut RES_QUEUE: BBBuffer<RES_CAPACITY> = BBBuffer::new();
    static mut URC_QUEUE: BBBuffer<URC_CAPACITY> = BBBuffer::new();
    let queues = atat::Queues {
        res_queue: unsafe { RES_QUEUE.try_split_framed().unwrap() },
        urc_queue: unsafe { URC_QUEUE.try_split_framed().unwrap() },
    };

    // Two timer instances
    let atat_timer = timer::SysTimer::new();
    let rp_timer = timer::SysTimer::new();

    // Atat client
    let config = atat::Config::new(atat::Mode::Timeout);
    let digester = atat::AtDigester::<URCMessages<URC_RX_SIZE>>::new();
    let (client, mut ingress) =
        atat::ClientBuilder::<_, _, _, TIMER_HZ, ATAT_RX_SIZE, RES_CAPACITY, URC_CAPACITY>::new(
            tx, atat_timer, digester, config,
        )
        .build(queues);

    // unwrap!(spawner.spawn(reader(rx, ingress)));
    unwrap!(spawner.spawn(reader(rx, 1)));

    // let mut client = MokoMkl62BaClient::new(client, rp_timer);
    // if let Err(e) = client.verify_com_is_working() {
    //     error!("Error verifying com is working: {:?}", e);
    // }
}

#[embassy_executor::task]
async fn reader(mut rx: BufferedUartRx<'static, UART1>, mut ingress: AtDigesterIngressManager) {
    info!("Reading...");
    loop {
        let mut index = 0;
        let mut total_buf = [0; 64];
        let mut read_buf = [0; 8];
        let mut read = rx.read(&mut read_buf).await.unwrap();
        while read != 0 {
            info!("Read {:?}", read);
            for i in 0..read {
                total_buf[i + index] = read_buf[i];
            }
            index += read;

            // Next read will overflow
            if index + 8 > 64 {
                let s = unsafe {
                    heapless::String::<64>::from_str(core::str::from_utf8_unchecked(&total_buf))
                        .unwrap()
                };
                // info!(">{:?}", s);
                index = 0;
                total_buf = [0; 64];
            }

            read = rx.read(&mut read_buf).await.unwrap();
        }
        let s = unsafe {
            heapless::String::<64>::from_str(core::str::from_utf8_unchecked(&total_buf)).unwrap()
        };
        // info!(">{:?}", s);
    }
}

mod timer {
    use core::convert::TryInto;
    use embassy_time::Instant as EmbInstant;

    use atat::clock::Clock;
    use fugit::Instant;

    /// A timer with millisecond precision.
    pub struct SysTimer {
        start: EmbInstant,
        duration_ms: u32,
        started: bool,
    }

    impl SysTimer {
        pub fn new() -> SysTimer {
            SysTimer {
                start: EmbInstant::now(),
                duration_ms: 0,
                started: false,
            }
        }
    }

    impl Clock<1000> for SysTimer {
        type Error = &'static str;

        /// Return current time `Instant`
        fn now(&mut self) -> fugit::TimerInstantU32<1000> {
            let milliseconds = (EmbInstant::now() - self.start).as_millis();
            let ticks: u32 = milliseconds.try_into().expect("u32 timer overflow");
            Instant::<u32, 1, 1000>::from_ticks(ticks)
        }

        /// Start timer with a `duration`
        fn start(&mut self, duration: fugit::TimerDurationU32<1000>) -> Result<(), Self::Error> {
            // (Re)set start and duration
            self.start = EmbInstant::now();
            self.duration_ms = duration.ticks();

            // Set started flag
            self.started = true;

            Ok(())
        }

        /// Tries to stop this timer.
        ///
        /// An error will be returned if the timer has already been canceled or was never started.
        /// An error is also returned if the timer is not `Periodic` and has already expired.
        fn cancel(&mut self) -> Result<(), Self::Error> {
            if !self.started {
                Err("cannot cancel stopped timer")
            } else {
                self.started = false;
                Ok(())
            }
        }

        /// Wait until timer `duration` has expired.
        /// Must return `nb::Error::WouldBlock` if timer `duration` is not yet over.
        /// Must return `OK(())` as soon as timer `duration` has expired.
        fn wait(&mut self) -> nb::Result<(), Self::Error> {
            let now = EmbInstant::now();
            if (now - self.start).as_millis() > self.duration_ms.into() {
                Ok(())
            } else {
                Err(nb::Error::WouldBlock)
            }
        }
    }

    //     #[cfg(test)]
    //     mod tests {
    //         use super::*;
    //
    //         #[test]
    //         fn test_delay() {
    //             let mut timer = SysTimer::new();
    //
    //             // Wait 500 ms
    //             let before = StdInstant::now();
    //             timer.start(fugit::Duration::<u32, 1, 1000>::from_ticks(500)).unwrap();
    //             nb::block!(timer.wait()).unwrap();
    //             let after = StdInstant::now();
    //
    //             let duration_ms = (after - before).as_millis();
    //             assert!(duration_ms >= 500);
    //             assert!(duration_ms < 1000);
    //         }
    //     }
}
