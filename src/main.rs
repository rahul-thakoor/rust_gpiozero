extern crate gpiozero;

use std::thread;
use std::time::Duration;

use gpiozero::output_devices::LED;

fn main() {
    
    let mut d = LED::new(17);
    d.blink(1,1);
    


}
