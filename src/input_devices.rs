use devices::GPIODevice;
use traits::*;
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


/// Represents a generic input device with typical on/off behaviour.
/// Adds machinery to fire the active and inactive events for devices 
/// that operate in a typical digital manner: straight forward on / off
/// states with (reasonably) clean transitions between the two.

pub struct DigitalInputDevice {
    pin : Pin
}

impl DigitalInputDevice{
    pub fn new(pin:u64) -> DigitalInputDevice {
        let inpin = InputDevice::new(pin);
        DigitalInputDevice { pin: inpin.pin }
    }
}

/// gives DigitalInputDevice Device behaviours such as close, is_active, etc
impl Device for DigitalInputDevice {
    fn pin(&self) -> Pin {
       self.pin
    }
}

/// Give DigitalInputDevice event traits 
impl EventsTrait for DigitalInputDevice {
    // add code here
    fn pin(&self) -> Pin {
       self.pin
    }
}