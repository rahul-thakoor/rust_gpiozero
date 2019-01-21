use rust_gpiozero::*;
use std::time::Duration;
use std::thread;


fn main() {
    // Create a new LED attached to Pin 17
    let mut led = PWMOutputDevice::new(17);
    // blink the LED
    

    led.on();
    //(1.0,1.0,1.0,1.0,5);
    thread::sleep(Duration::from_secs(100));
    
    

}