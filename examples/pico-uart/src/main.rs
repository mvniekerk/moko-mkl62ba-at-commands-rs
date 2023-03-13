#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_executor::_export::StaticCell;
use embassy_rp::interrupt;
use embassy_rp::peripherals::{UART1};
use embassy_rp::uart::{BufferedUart, BufferedUartRx, Config};
use embassy_time::{Duration, Timer};
use embedded_io::asynch::{Read, Write};
use {defmt_rtt as _, panic_probe as _};

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
    let (tx_pin, rx_pin, uart) = (p.PIN_8, p.PIN_9, p.UART1);

    let irq = interrupt::take!(UART1_IRQ);
    let tx_buf = &mut singleton!([0u8; 16])[..];
    let rx_buf = &mut singleton!([0u8; 16])[..];
    let mut config = Config::default();
    config.baudrate = 9600;
    // config.baudrate = 921600;
    // config.baudrate = 115200;
    let mut uart = BufferedUart::new(uart, irq, tx_pin, rx_pin, tx_buf, rx_buf, Config::default());
    let (rx, mut tx) = uart.split();

    unwrap!(spawner.spawn(reader(rx)));

    info!("Writing...");
    loop {
        let data = b"AT\r\n";
        info!("TX {:?}", data);
        tx.write_all(data).await.unwrap();
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn reader(mut rx: BufferedUartRx<'static, UART1>) {
    info!("Reading...");
    loop {
        let mut buf = [0; 31];
        rx.read_exact(&mut buf).await.unwrap();
        info!("RX {:?}", buf);
    }
}