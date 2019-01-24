use rust_gpiozero::*;
use std::time::Duration;
use std::thread;


fn main() {
    // Create a new LED attached to Pin 17
    let mut led = PWMOutputDevice::new(17);
    // blink the LED repeatedly
    

    led.pulse(3.0,1.0,3.0,3.0);
    println!("pin {}",led.pin());

    thread::sleep(Duration::from_secs(25));
    led.stop();
    thread::sleep(Duration::from_secs(10));



}