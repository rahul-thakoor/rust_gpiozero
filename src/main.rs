extern crate gpiozero;

use std::thread;
use std::time::Duration;

use gpiozero::output_devices::*;


fn main() {
    
    /* let mut d = LED::new(17);
    d.on();
    thread::sleep(Duration::from_secs(3));
    d.off(); */

    let mut d = Buzzer::new(17);
    d.beep(2,3);

    

}
