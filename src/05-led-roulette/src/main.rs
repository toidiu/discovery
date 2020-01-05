#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    // infinite loop; just so we don't leave this stack frame
    let half_period = 100_u16;

    let mut idx: usize = 0;
    clear_leds(&mut leds);
    loop {
        // clear_leds(&mut leds);
        turn_off_led(&mut leds, idx);
        increment_idx(&mut idx);
        turn_on_led(&mut leds, idx);

        delay.delay_ms(half_period);
    }
}

fn increment_idx(idx: &mut usize) {
    *idx = (*idx + 1) % 8;
}

fn clear_leds(leds: &mut Leds) {
    for idx in 0..8 {
        leds[idx].off();
    }
}

fn turn_off_led(leds: &mut Leds, idx: usize) {
    leds[idx].off();
}

fn turn_on_led(leds: &mut Leds, idx: usize) {
    leds[idx].on();
}
