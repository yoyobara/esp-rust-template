use core::fmt::Write;

use embassy_time::{Duration, Timer};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Baseline, Text};
use esp_hal::gpio::Output;
use heapless::String;

use crate::board::Oled;

#[embassy_executor::task]
pub async fn blink(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
pub async fn show_text(mut oled: Oled) {
    let display = &mut oled.display;
    let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    let mut seconds: u32 = 0;
    loop {
        // Draw into the RAM framebuffer, then push it to the panel in one go.
        display.clear_buffer();

        Text::with_baseline("Hello, Rust!", Point::zero(), style, Baseline::Top)
            .draw(display)
            .unwrap();

        let mut uptime: String<16> = String::new();
        write!(uptime, "up: {seconds}s").unwrap();
        Text::with_baseline(&uptime, Point::new(0, 16), style, Baseline::Top)
            .draw(display)
            .unwrap();

        display.flush().await.unwrap();

        seconds += 1;
        Timer::after(Duration::from_secs(1)).await;
    }
}
