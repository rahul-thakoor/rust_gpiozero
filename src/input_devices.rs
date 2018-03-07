//! Input device component interfaces for devices such as `Button`
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

    /// Returns a value representing the device's state.
    fn value(&self) -> i8 {
        
        let value =  self.pin.get_value().expect("Could not check if device is active");
        value as i8
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
    /// Create a new Digital Input Device
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

    /// Returns a value representing the device's state.
    fn value(&self) -> i8 { 
        let value =  self.pin.get_value().expect("Could not check if device is active");
        value as i8
    }
}

/// Give DigitalInputDevice event traits 
impl EventsTrait for DigitalInputDevice {}


/// Represents a simple push button or switch.
/// Connect one side of the button to a ground pin, and the other to any GPIO pin

pub struct Button {
    pin: Pin
}

impl Device for Button {
    fn pin(&self) -> Pin {
       self.pin
    }
    /// Returns a value representing the device's state.
    fn value(&self) -> i8 { 
        let value =  self.pin.get_value().expect("Could not check if device is active");
        value as i8
    }
}

impl EventsTrait for Button {}

impl Button {
    /// Create a new Button
    pub fn new(pin:u64) -> Button{
        let din = DigitalInputDevice::new(pin);
        Button{
            pin : din.pin
        }
    }

    /// Pause the script until the device is activated
    pub fn wait_for_press(&self){
        self.wait_for_active();
    }

    /// Pause the script until the device is deactivated
    pub fn wait_for_release(&self){
        self.wait_for_inactive();
    }


}    


/// Represents a generic input device which takes its value 
/// from the average of a queue of historical values.

pub struct SmoothedInputDevice;

// Todo implement SmoothedInputDevice
