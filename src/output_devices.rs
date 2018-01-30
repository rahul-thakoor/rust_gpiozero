
use devices::GPIODevice;
use sysfs_gpio::{Direction, Pin};

#[derive(Debug)]
pub struct OutputDevice {
    pub gpio: GPIODevice
}



impl OutputDevice {
    pub fn new(pin:u64) -> OutputDevice{
        let gpiodevice = GPIODevice::new(pin);
        // set direction to output
        gpiodevice.pin.set_direction(Direction::Out).expect("Could not set pin to Output mode");
        OutputDevice {gpio:gpiodevice}
    }


    /// Turns the device on.

    pub fn on(&mut self){
        self.gpio.pin.set_value(1).expect("Could not turn pin ON");
    }

    /// Turns the device off.
    pub fn off(&mut self){
        self.gpio.pin.set_value(0).expect("Could not turn pin OFF");
    }
    
    /// Reverse the state of the device. If it's on, turn it off; if it's off,
    /// turn it on.
    pub fn toggle(&mut self) {
        match self.gpio.pin.get_value() {
            Ok(value) => if value == 1 { self.off() } else { self.on() },
            Err(e) => println!("error toggling pin: {:?}", e),
        }
    }
}