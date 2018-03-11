extern crate gpiozero;

use std::thread;
use std::time::Duration;

use gpiozero::*;



fn main() {

    let mut motor = Motor::new(17,27);
    motor.forward();
    thread::sleep(Duration::from_secs(3));
    motor.stop();

}
