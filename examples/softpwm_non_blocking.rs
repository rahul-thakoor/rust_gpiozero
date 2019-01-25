use rust_gpiozero::*;
use std::io;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a new LED attached to Pin 17
    let mut led = PWMOutputDevice::new(17);
    // blink the LED repeatedly

    // led.pulse(3.0, 1.0, 3.0, 3.0);

    led.on();
    for _ in 0..5 {
        // println!("value {}", led.is_active());
        led.toggle();
        thread::sleep(Duration::from_secs(2));
    }
    thread::sleep(Duration::from_secs(8));
    led.set_value(0.2);
    thread::sleep(Duration::from_secs(3));

    led.pulse(3.0, 3.0, Some(5));
    led.pulse(2.0, 2.0, None);

    // wait for key press
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
