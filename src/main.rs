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

    let mut input = InputDevice::new(17);
    println!("{}",input.is_active());

    

}
