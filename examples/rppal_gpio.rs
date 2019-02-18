use rust_gpiozero::*;
use std::io;
use std::io::prelude::*;
use std::thread;

use std::time::Duration;
fn main() {
    // Create a button which is attached to Pin 17

    /*     let mut o = OutputDeviceR::new(17);

    o.set_active_high(false);
    o.off();
    println!("{}", o.is_active());
    thread::sleep(Duration::from_secs(3));

    o.on();
    println!("{}", o.is_active());
    thread::sleep(Duration::from_secs(3)); */

    //let mut led = DigitalOutputDeviceR::new(17);

    // led.blink(1.0, 1.0, Some(5));
    // led.wait();

    // let mut input = DigitalInputDeviceR::new_with_pullup(2);
    let mut input = ButtonR::new(23);
    input.wait_for_release(None);
    println!("released");



    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
