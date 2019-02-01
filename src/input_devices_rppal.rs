//! Input device component interfaces for devices such as `Button`
use rppal::gpio::{Gpio, InputPin, Level, Mode};

/// Represents a generic GPIO input device.
#[derive(Debug)]
pub struct InputDeviceR {
    pin: InputPin,
    active_state: bool,
    inactive_state: bool,
}

impl InputDeviceR {
    /// Returns an InputDevice with the pin number given with the pin pulled to low by default
    /// `is_active` property is adjusted accordingly so that
    /// ``True`` still means active regardless of the :attr:`pull_up` setting
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin which the device is attached to
    ///  
    pub fn new(pin:u8) -> InputDeviceR{
        match Gpio::new() {
            Err(e) => panic!("{:?}", e),
            Ok(gpio) => match gpio.get(pin) {
                Err(e) => panic!("{:?}", e),
                Ok(pin) => InputDeviceR {
                    pin: pin.into_input_pulldown(),
                    active_state: true,
                    inactive_state: false,
                },
            },
        }
    }
    /// Returns an InputDevice with the pin number given with the pin pulled high with an internal resistor by default
    /// `is_active` property is adjusted accordingly so that
    /// ``True`` still means active regardless of the :attr:`pull_up` setting
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin which the device is attached to
    ///  
    pub fn new_with_pullup(pin:u8) -> InputDeviceR{
        match Gpio::new() {
            Err(e) => panic!("{:?}", e),
            Ok(gpio) => match gpio.get(pin) {
                Err(e) => panic!("{:?}", e),
                Ok(pin) => InputDeviceR {
                    pin: pin.into_input_pullup(),
                    active_state: false,
                    inactive_state: true,
                },
            },
        }
    }
    
    impl_device!();
    impl_gpio_device!();
    impl_io_device!();
}