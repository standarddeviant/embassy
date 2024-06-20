//! This example shows the ease of debouncing a button with async rust.
//! Hook up a button or switch between pin 9 and ground.

#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level::{self, Low, High}, Output, Pull};
use embassy_time::{with_deadline, Duration, Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

pub struct Debouncer<'a> {
    input: Input<'a>,
    debounce: Duration,
}

impl<'a> Debouncer<'a> {
    pub fn new(input: Input<'a>, debounce: Duration) -> Self {
        Self { input, debounce }
    }

    pub async fn debounce(&mut self) -> Level {
        loop {
            let l1 = self.input.get_level();

            self.input.wait_for_any_edge().await;

            Timer::after(self.debounce).await;

            let l2 = self.input.get_level();
            if l1 != l2 {
                break l2;
            }
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut btn = Debouncer::new(Input::new(p.PIN_19, Pull::Up), Duration::from_millis(20));
    let mut led = Output::new(p.PIN_25, Low);
    let mut prev_level = Low;
    let mut start = Instant::now();

    info!("Debounce Demo");

    loop {
        // wait for button press
        let cur_level = btn.debounce().await;
        match (prev_level, cur_level) {
            (High, Low) => {
                start = Instant::now();
                led.set_level(High);
                info!("Button Pressed");
            },

            (Low, High) => {
                led.set_level(Low);
                info!("Button released @ {} ms", start.elapsed().as_millis());
            },

            _ => {} // ignore other cases
        }
        prev_level = cur_level;
    }
}

