use std::thread;
use std::time::Duration;

use sysfs_gpio::{Direction, Pin};

#[derive(Debug)]
pub struct Led {
    pin: Pin
}

impl Led {
    pub fn new(pin:u64) -> Led{
        let mut led_gpio = Pin::new(pin);
        led_gpio.export().expect("Could not export the selected gpio");
        led_gpio.set_direction(Direction::Out).expect("Could not set led pin to OUTPUT mode");
        Led{
            pin: led_gpio
        }
    }

    pub fn on(&self){
        self.pin.set_value(1).expect("Could not turn led pin ON");
    }

    pub fn off(&self){
        self.pin.set_value(0).expect("Could not turn led pin OFF");
    }
}
