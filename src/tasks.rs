use embassy_time::{Duration, Timer};
use esp_hal::gpio::Output;

#[embassy_executor::task]
pub async fn blink(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(500)).await;
    }
}
