extern crate gpiozero;

use std::thread;
use std::time::Duration;

use gpiozero::input_devices::*;
use gpiozero::output_devices::*;
use gpiozero::traits::*;




fn main() {
    
    /* let mut d = LED::new(17);
    d.on();
    thread::sleep(Duration::from_secs(3));
    d.off(); */

    /* let button = Button::new(17);
    button.wait_for_press();
    println!("button pressed"); */

    let mut motor = Motor::new(17,27);
    motor.devices.forward.on();
    

}
