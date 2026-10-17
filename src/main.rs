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

mod board;
mod tasks;

use embassy_executor::Spawner;
use esp_backtrace as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    let board = board::init();

    spawner.spawn(tasks::blink(board.led).unwrap());
}
