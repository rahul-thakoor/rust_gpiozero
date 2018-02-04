use devices::GPIODevice;
use traits::Device;
use sysfs_gpio::{Direction,Pin};


/// Represents a generic GPIO input device.
#[derive(Debug)]
pub struct InputDevice {
    pub pin : Pin
}


impl InputDevice {
    /// Creates a new input device with pin number `pin`
    pub fn new(pin:u64) -> InputDevice{
        let gpiodevice = GPIODevice::new(pin);
        // set direction to input
        gpiodevice.pin.set_direction(Direction::In).expect("Could not set pin to Input mode");
        InputDevice {
            pin: gpiodevice.pin
            }
    }
}

impl Device for InputDevice {
    fn pin(&self) -> Pin {
       self.pin
    }
}
