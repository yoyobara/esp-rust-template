//! Chip and runtime bring-up for the Heltec WiFi Kit 32 V3.

use esp_hal::Async;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use log::info;
use ssd1306::mode::BufferedGraphicsModeAsync;
use ssd1306::prelude::*;
use ssd1306::{I2CDisplayInterface, Ssd1306Async};

/// The onboard 128x64 SSD1306, driven over async I2C with a framebuffer.
pub type Display = Ssd1306Async<
    I2CInterface<I2c<'static, Async>>,
    DisplaySize128x64,
    BufferedGraphicsModeAsync<DisplaySize128x64>,
>;

/// The OLED together with the pins that must stay driven while it is in use.
pub struct Oled {
    pub display: Display,
    /// Vext power switch (GPIO36, active low).
    _vext: Output<'static>,
    /// OLED reset line (GPIO21, active low), held high after reset.
    _rst: Output<'static>,
}

/// Board resources handed out to tasks.
pub struct Board {
    /// Onboard white LED (GPIO35, active high).
    pub led: Output<'static>,
    /// Onboard OLED, reset and initialized.
    pub oled: Oled,
}

/// Initializes logging, the HAL and the esp-rtos runtime, then claims the board's peripherals.
pub async fn init() -> Board {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);

    info!("Embassy initialized!");

    // Vext powers the OLED; give it a moment to come up before talking to it.
    let vext = Output::new(peripherals.GPIO36, Level::Low, OutputConfig::default());
    embassy_time::Timer::after_millis(10).await;

    let i2c = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_sda(peripherals.GPIO17)
    .with_scl(peripherals.GPIO18)
    .into_async();

    let mut display = Ssd1306Async::new(
        I2CDisplayInterface::new(i2c),
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();

    let mut rst = Output::new(peripherals.GPIO21, Level::High, OutputConfig::default());
    display
        .reset(&mut rst, &mut embassy_time::Delay)
        .await
        .unwrap();
    display.init().await.unwrap();

    info!("OLED initialized!");

    Board {
        led: Output::new(peripherals.GPIO35, Level::Low, OutputConfig::default()),
        oled: Oled {
            display,
            _vext: vext,
            _rst: rst,
        },
    }
}
