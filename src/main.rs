#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

mod screen;

use embedded_graphics::{
    Drawable as _,
    draw_target::DrawTarget as _,
    geometry::Point,
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::{Rgb565, RgbColor as _},
    text::{Baseline, Text},
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    peripherals::Peripherals,
    spi::{
        Mode,
        master::{Config, Spi},
    },
    time::Rate,
};
use log::info;

use crate::screen::Screen;

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
    let mut delay = Delay::new();

    let spi = Spi::new(
        p.SPI2,
        Config::default()
            .with_frequency(Rate::from_mhz(40))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(p.GPIO40)
    .with_mosi(p.GPIO41);

    let cs = Output::new(p.GPIO42, Level::High, OutputConfig::default());
    let dc = Output::new(p.GPIO47, Level::Low, OutputConfig::default());
    let rst = Output::new(p.GPIO48, Level::Low, OutputConfig::default());
    let mut bl = Output::new(p.GPIO33, Level::Low, OutputConfig::default()); // backlight

    let spi_device = ExclusiveDevice::new(spi, cs, delay).unwrap();
    let mut buffer = [0u8; 512];
    let mut screen = Screen::init(spi_device, dc, rst, &mut buffer, &mut delay).unwrap();

    screen.clear(Rgb565::BLACK).unwrap();
    Text::with_baseline(
        "nigga what",
        Point::new(0, 0),
        MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE),
        Baseline::Top,
    )
    .draw(&mut screen)
    .unwrap();

    bl.set_high();

    loop {}
}
