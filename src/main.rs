#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

// generator version: 1.4.0
// generator parameters: -o esp32s3 -o unstable-hal -o embassy -o log -o esp-backtrace --toolchain esp

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    peripherals::Peripherals,
};
use log::info;

esp_bootloader_esp_idf::esp_app_desc!();

fn init() -> Peripherals {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let p = esp_hal::init(config);

    info!("Embassy initialized!");

    p
}

#[esp_hal::main]
fn main() -> ! {
    let p = init();
    let mut led1 = Output::new(p.GPIO45, Level::High, OutputConfig::default());
    let mut led2 = Output::new(p.GPIO46, Level::Low, OutputConfig::default());
    let delay = Delay::new();

    loop {
        led1.toggle();
        led2.toggle();

        delay.delay_millis(500);
    }
}
