use rust_gpiozero::*;
use std::time::Duration;
use std::thread;


fn main() {
    // Create a new LED attached to Pin 17
    let mut led = PWMOutputDeviceNB::new(17);
    // blink the LED
    

    led.blink(3.0,2.0,5.0,3.0,20);
    println!("dcfdsf");
    thread::sleep(Duration::from_secs(10));
    led.stop();
    println!("hi2");
    thread::sleep(Duration::from_secs(25));

}