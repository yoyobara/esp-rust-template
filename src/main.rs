#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

// generator version: 1.4.0
// generator parameters: -o esp32s3 -o unstable-hal -o log -o esp-backtrace --toolchain esp

use core::fmt::Write;

use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Baseline, Text};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::main;
use esp_hal::time::Rate;
use heapless::String;
use log::info;
use ssd1306::prelude::*;
use ssd1306::{I2CDisplayInterface, Ssd1306};
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
#[allow(
    clippy::large_stack_frames,
    reason = "the OLED's 1 KiB framebuffer lives on main's stack, which is far larger"
)]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    let mut delay = Delay::new();

    // Vext (GPIO36, active low) powers the OLED; give it a moment to come up.
    let _vext = Output::new(peripherals.GPIO36, Level::Low, OutputConfig::default());
    delay.delay_millis(10);

    let i2c = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_sda(peripherals.GPIO17)
    .with_scl(peripherals.GPIO18);

    let mut display = Ssd1306::new(
        I2CDisplayInterface::new(i2c),
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();

    let mut rst = Output::new(peripherals.GPIO21, Level::High, OutputConfig::default());
    display.reset(&mut rst, &mut delay).unwrap();
    display.init().unwrap();

    info!("OLED initialized!");

    let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    let mut seconds: u32 = 0;
    loop {
        // Draw into the RAM framebuffer, then push it to the panel in one go.
        display.clear_buffer();

        Text::with_baseline("Hello, Rust!", Point::zero(), style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        let mut uptime: String<16> = String::new();
        write!(uptime, "up: {seconds}s").unwrap();
        Text::with_baseline(&uptime, Point::new(0, 16), style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        seconds += 1;
        delay.delay_millis(1000);
    }
}
