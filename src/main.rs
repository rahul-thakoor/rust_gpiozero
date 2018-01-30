extern crate gpiozero;

use std::thread::sleep;
use std::time::Duration;

use gpiozero::devices::GPIODevice;


fn main() {
    
    let d:Device = GPIODevice::new(17);

    d.test();
}
