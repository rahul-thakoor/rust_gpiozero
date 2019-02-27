//! Describes generic devices such as `GPIODevice` and `CompositeDevice`

use rppal::gpio::{Gpio, Level, Pin};

/// Represents a single device of any type; GPIO-based, SPI-based, I2C-based,
/// etc.  It defines the basic services applicable to all devices
pub trait Device {
    /// Shut down the device and release all associated resources.
    fn close(self);

    /// Returns ``True`` if the device is currently active and ``False`` otherwise.
    fn is_active(&self) -> bool;
}
#[macro_export]
macro_rules! impl_device {
    () => {
    /// Returns ``True`` if the device is currently active and ``False`` otherwise.
    pub fn is_active(&self) -> bool {
        self.value()
    }
    /// Shut down the device and release all associated resources.
    pub fn close(self) {
        drop(self)
    }
    }
}

/// Represents a generic GPIO device and provides the services common to all single-pin GPIO devices
#[derive(Debug)]
pub struct GpioDevice {
    pin: Pin,
    active_state: bool,
    inactive_state: bool,
}

macro_rules! impl_gpio_device {
    () => {
    /// The `Pin` that the device is connected to.
    pub fn pin(&self) -> u8 {
        self.pin.pin()
    }

    }
}

impl GpioDevice {
    /// Returns a GpioDevice with the pin number given
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin which the device is attached to
    ///
    pub fn new(pin: u8) -> GpioDevice {
        match Gpio::new() {
            Err(e) => panic!("{:?}", e),
            Ok(gpio) => match gpio.get(pin) {
                Err(e) => panic!("{:?}", e),
                Ok(pin) => GpioDevice {
                    pin,
                    active_state: true,
                    inactive_state: false,
                },
            },
        }
    }

    /// Returns a value representing the device's state.
    pub fn value(&self) -> bool {
        self.state_to_value()
    }

    fn state_to_value(&self) -> bool {
        match self.pin.read() {
            Level::High => self.active_state,
            Level::Low => !self.active_state,
        }
    }

    impl_device!();
    impl_gpio_device!();
}
