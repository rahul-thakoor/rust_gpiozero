//! Blinks an LED : on_time: 2 seconds and off_time: 3 seconds


use rust_gpiozero::*;

fn main() {

    // Create a new LED attached to Pin 17
    let mut led = LED::new(17);
    // blink the LED
    led.blink(2,3);

  }
