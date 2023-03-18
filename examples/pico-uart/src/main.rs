#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

extern crate alloc;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use core::str::FromStr;
use defmt::{error, info, unwrap};
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
use embassy_time::{Duration, Timer};
use embedded_alloc::Heap;
use moko_mkl62ba_at_commands::client::MokoMkl62BaClient;
use moko_mkl62ba_at_commands::lora::responses::LoraReceivedBytes;
use moko_mkl62ba_at_commands::lora::types::{LoraJoinMode, LoraJoiningStatus, LoraRegion};
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

// type AtDigesterIngressManager = u32;
type AtDigesterIngressManager =
    IngressManager<AtDigester<URCMessages<URC_RX_SIZE>>, ATAT_RX_SIZE, RES_CAPACITY, URC_CAPACITY>;

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

    unwrap!(spawner.spawn(reader(rx, ingress)));
    // unwrap!(spawner.spawn(reader(rx, 1)));

    let mut client = MokoMkl62BaClient::new(client, rp_timer);
    if let Err(e) = client.verify_com_is_working() {
        error!("Error verifying com is working: {:?}", e);
    } else {
        info!("Com is working");
    }

    if let Err(e) = client.at_echo_set(false) {
        error!("Error setting echo: {:?}", e);
    } else {
        info!("Echo set to false");
    }

    if let Err(e) = client.join_mode_set(LoraJoinMode::Otaa) {
        error!("Error setting join mode: {:?}", e);
    } else {
        info!("Join mode set to OTAA");
    }

    if let Err(e) = client.dev_eui_set(0x0123456789ABCDEF) {
        error!("Error setting dev eui: {:?}", e);
    } else {
        info!("Dev EUI set");
    }

    if let Err(e) = client.app_eui_set(0x0) {
        error!("Error setting app eui: {:?}", e);
    } else {
        info!("App EUI set");
    }

    if let Err(e) = client.app_key_set(0x1233456789ABCDEF_0123456789ABCDEF) {
        error!("Error setting app key: {:?}", e);
    } else {
        info!("App key set");
    }

    if let Err(e) = client.lora_region_set(LoraRegion::Eu868) {
        error!("Error setting lora region: {:?}", e);
    } else {
        info!("Lora region set");
    }

    if let Err(e) = client.auto_join_set(true) {
        error!("Error setting auto join: {:?}", e);
    } else {
        info!("Auto join set");
    }

    if let Err(e) = client.lora_join_otaa() {
        error!("Error joining: {:?}", e);
    } else {
        info!("Started joining OTAA");
    }

    let mut joined = false;
    for i in 0..100 {
        let status = client.lora_join_status();
        match status {
            Ok(status) => match status {
                LoraJoiningStatus::Joined => {
                    info!("Joined");
                    joined = true;
                    break;
                }
                LoraJoiningStatus::Joining => {
                    info!("Joining");
                }
                LoraJoiningStatus::JoinFailed => {
                    info!("Join failed");
                }
                LoraJoiningStatus::InAbpModeError => {
                    error!("In ABP mode");
                }
                LoraJoiningStatus::BusyError => {
                    error!("Busy");
                }
                LoraJoiningStatus::Unknown => {
                    error!("Unknown error");
                }
            },

            Err(e) => {
                error!("Error getting join status: {:?}", e);
            }
        }
        Timer::after(Duration::from_secs(1)).await;
    }

    if !joined {
        error!("Failed to join");
        return;
    }

    match client.send(3, 12, b"Hello from Moko") {
        Ok(_d) => {
            info!("Sent bytes");
        }
        Err(e) => error!("Error sending: {:?}", e),
    }
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
                index = 0;
                total_buf = [0; 64];
            }

            read = rx.read(&mut read_buf).await.unwrap();
        }
        let s = unsafe {
            heapless::String::<64>::from_str(core::str::from_utf8_unchecked(&total_buf)).unwrap()
        };
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
