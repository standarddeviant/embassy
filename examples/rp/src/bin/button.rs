//! This example uses the RP Pico on board LED to test input pin 28. This is not the button on the board.
//!
//! It does not work with the RP Pico W board. Use wifi_blinky.rs and add input pin.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{self, Input, Level::{Low, High}, Output, Pull};
use embedded_hal_1::digital::InputPin;
use {defmt_rtt as _, panic_probe as _};


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Low);
    let mut prev_level = Low;

    // Use PIN_28, Pin34 on J0 for RP Pico, as a input.
    // You need to add your own button.
    let button = Input::new(p.PIN_19, Pull::Up);

    loop {
        // get button level + set led
        let cur_level = button.get_level();
        let led_level = match cur_level {
            Low => High,
            High => Low
        };
        led.set_level(led_level);

        // process prev_level vs. cur_level)
        let prev_cur = (prev_level, cur_level);
        match prev_cur {
            (Low, High) => {
                info!("Low to High!");
            },
            (High, Low) => {
                info!("High to Low!");
            },
            // ignore other cases
            _ => {}
        }
        prev_level = cur_level;
    }
}
