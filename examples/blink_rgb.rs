//! Blinks an LED : on_time: 2 seconds and off_time: 3 seconds

use rust_gpiozero::*;

fn main() {
    // Create a new LED attached to Pin 17
    let mut led = RGBLED::new(23, 24, 25);

    // on_time = 2 secs, off_time=3 secs
    led.red.blink(0.11, 0.05);
    led.green.blink(0.7, 0.17);
    led.blue.blink(0.23, 0.03);

    // prevent program from exiting immediately
    led.red.wait();
    led.green.wait();
    led.blue.wait();
}
