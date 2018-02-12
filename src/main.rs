extern crate gpiozero;

use std::thread;
use std::time::Duration;

use gpiozero::input_devices::*;
use gpiozero::traits::*;


fn main() {
    
    /* let mut d = LED::new(17);
    d.on();
    thread::sleep(Duration::from_secs(3));
    d.off(); */

    let mut input = DigitalInputDevice::new(17);
    input.wait_for_inactive();
    println!("input deactivated");

    

}
