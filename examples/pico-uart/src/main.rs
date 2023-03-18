#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_executor::_export::StaticCell;
use embassy_rp::interrupt;
use embassy_rp::peripherals::{UART1};
use embassy_rp::uart::{BufferedUart, BufferedUartRx, Config, Parity, StopBits};
use embassy_time::{Duration, Timer};
use embedded_io::asynch::{Read, Write};
use {defmt_rtt as _, panic_probe as _};
use core::str::FromStr;
use embassy_rp::uart::DataBits::DataBits8;
use moko_mkl62ba_at_commands::general::VerifyComIsWorking;

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
    let tx_buf = &mut singleton!([0u8; 64])[..];
    let rx_buf = &mut singleton!([0u8; 255])[..];
    let mut config = Config::default();
    config.baudrate = 9600;
    config.parity = Parity::ParityNone;
    config.stop_bits = StopBits::STOP1;
    config.data_bits = DataBits8;
    let mut uart = BufferedUart::new(uart, irq, tx_pin, rx_pin, tx_buf, rx_buf, config);
    let (rx, mut tx) = uart.split();

    unwrap!(spawner.spawn(reader(rx)));

    let verify_com_is_working = VerifyComIsWorking {};

    loop {
        for _i in 0..10 {
            let data = b"AT+RESET\r\n";
            info!("TX {:X}", data);
            // uart.write_all(data).await.unwrap();
            tx.write_all(data).await.unwrap();

            Timer::after(Duration::from_secs(5)).await;

        }
        info!(".");
    }
}

#[embassy_executor::task]
async fn reader(mut rx: BufferedUartRx<'static, UART1>) {
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
                let s = unsafe { heapless::String::<64>::from_str(core::str::from_utf8_unchecked(&total_buf)).unwrap() };
                info!(">{:?}", s);
                index = 0;
                total_buf = [0; 64];
            }

            read = rx.read(&mut read_buf).await.unwrap();
        }
        let s = unsafe { heapless::String::<64>::from_str(core::str::from_utf8_unchecked(&total_buf)).unwrap() };
        info!(">{:?}", s);
    }
}