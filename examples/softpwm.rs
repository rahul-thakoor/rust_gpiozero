use rust_gpiozero::*;
use std::io;
use std::io::prelude::*;

fn main() {
    // Create a new LED attached to Pin 17
    let mut led = PWMLED::new(17);

    // blink the LED 5 times
    led.set_blink_count(5);
    led.blink(2.0, 2.0, 1.0, 1.0);

    // wait for key press to exit
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
