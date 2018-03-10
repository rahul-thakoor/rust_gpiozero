extern crate gpiozero;
use gpiozero::*;

fn main() {
    
    // Create a new LED attached to Pin 14
    let mut led = LED::new(14);
    // blink the LED
    led.blink();

  }