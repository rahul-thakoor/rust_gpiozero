
use devices::GPIODevice;
use sysfs_gpio::{Direction, Pin};

#[derive(Debug)]
struct OutputDevice {
    pin: GPIODevice
}



impl OutputDevice {
    fn new(pin:u64) -> OutputDevice{
        let mut gpio = GPIODevice::new(pin);
        // set direction to output
        gpio.pin.set_direction(Direction::Out).expect("Could not set pin to Output mode");
        OutputDevice {pin:gpio}
    }

    ///Returns True if the device is currently active and False otherwise. 
   
    fn value(&self) -> bool{
        if self.pin.get_value() == 1 {
            return true
        } else{
            return false;
        }
    }

    /// Turns the device on.

    fn on(&mut self){
        self.pin.set_value(1).expect("Could not turn pin ON");
    }

    /// Turns the device off.
     fn off(&mut self){
        self.pin.set_value(0).expect("Could not turn pin OFF");
    }
    
    /// Reverse the state of the device. If it's on, turn it off; if it's off,
    /// turn it on.
    fn toggle(&mut self){
        if self.pin.get_value() == 1{
            self.off()
        }
        else{
            self.on()
        }
    }
}