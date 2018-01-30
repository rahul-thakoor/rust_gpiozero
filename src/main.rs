extern crate gpiozero;

use std::thread::sleep;
use std::time::Duration;

use gpiozero::devices::GPIODevice;


fn main() {
    
    let d = GPIODevice::new(17);
    println!("{}",d.pin.get_pin());
}
