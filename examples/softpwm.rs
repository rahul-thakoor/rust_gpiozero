use rust_gpiozero::*;
use std::time::Duration;
use std::thread;


fn main() {
    // Create a new LED attached to Pin 17
    let mut led = PWMOutputDevice::new(17);
    // blink the LED
    loop {
        led.blink(2.0,2.0,1.0,1.0);
    }
    

}