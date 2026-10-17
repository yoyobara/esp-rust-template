//! Chip and runtime bring-up for the Heltec WiFi Kit 32 V3.

use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::timer::timg::TimerGroup;
use log::info;

/// Board resources handed out to tasks.
pub struct Board {
    /// Onboard white LED (GPIO35, active high).
    pub led: Output<'static>,
}

/// Initializes logging, the HAL and the esp-rtos runtime, then claims the board's peripherals.
pub fn init() -> Board {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);

    info!("Embassy initialized!");

    Board {
        led: Output::new(peripherals.GPIO35, Level::Low, OutputConfig::default()),
    }
}
