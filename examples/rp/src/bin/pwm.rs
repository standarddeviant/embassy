//! This example shows how to use PWM (Pulse Width Modulation) in the RP2040 chip.
//!
//! The LED on the RP Pico W board is connected differently. Add a LED and resistor to another pin.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::pwm::{self, Pwm};
use embassy_time::{Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // let's ask python to do some math, mkay...
    // np.logspace(np.log10(50), np.log10(30e3), base=10, num=100).astype('uint16')
    let cbs: [u16; 100] = [
          49,    53,    56,    60,    64,    69,    73,    78,    83,
          89,    95,   101,   108,   115,   123,   131,   140,   149,
         159,   170,   182,   194,   207,   221,   235,   251,   268,
         286,   305,   325,   347,   370,   395,   421,   449,   479,
         511,   546,   582,   621,   662,   707,   754,   804,   858,
         915,   976,  1042,  1111,  1185,  1264,  1349,  1439,  1535,
        1638,  1747,  1864,  1988,  2121,  2262,  2413,  2574,  2746,
        2930,  3125,  3334,  3556,  3794,  4047,  4317,  4605,  4913,
        5241,  5591,  5964,  6362,  6787,  7240,  7723,  8239,  8789,
        9375, 10001, 10669, 11381, 12140, 12951, 13815, 14737, 15721,
       16771, 17890, 19084, 20358, 21717, 23167, 24713, 26363, 28122,
       30000
    ];

    let mut c = pwm::Config::default();
    c.top = 0x8000;
    // let mut pwm = Pwm::new_output_b(p.PWM_SLICE4, p.PIN_25, c.clone());
    let mut pwm = Pwm::new_output_b(p.PWM_SLICE2, p.PIN_21, c.clone());

    let start = Instant::now();
    let mut loop_iter: usize = 0;
    loop {
        info!("starting iter {} @ t = {:?}", loop_iter, start.elapsed());
        for cb in &cbs {
            Timer::after_millis(50).await;
            c.compare_b = *cb;
            pwm.set_config(&c);
        }
        for cb in cbs.iter().rev() {
            Timer::after_millis(50).await;
            c.compare_b = *cb;
            pwm.set_config(&c);
        }
        loop_iter += 1;
    }
}
