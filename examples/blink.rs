//! Blinks an LED : on_time: 2 seconds and off_time: 3 seconds

use rust_gpiozero::*;

fn main() {
    // Create a new LED attached to Pin 17
    let mut led = LED::new(17);

    // on_time = 2 secs, off_time=3 secs
    led.blink(2.0, 3.0);

    // prevent program from exiting immediately
    led.wait();
}
