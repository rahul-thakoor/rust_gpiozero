//! Describes generic devices such as `GPIODevice` and `CompositeDevice`
use crate::traits::Device;
use rppal::gpio::{Gpio, Level, Pin as PinR};
use sysfs_gpio::Pin;

/// Represents a generic GPIO device and provides the services common to all single-pin GPIO devices
#[derive(Debug)]
pub struct GPIODevice {
    pub pin: Pin,
}

impl GPIODevice {
    pub fn new(pin: u64) -> GPIODevice {
        //Create a new Pin with the provided pin_num
        let gpio = Pin::new(pin);
        //check if pin is not already exported

        //try to export the selected pin
        //Todo implement better error handling
        gpio.export().expect("Could not export the selected gpio");
        GPIODevice { pin: gpio }
    }
}

impl Device for GPIODevice {
    fn pin(&self) -> Pin {
        self.pin
    }

    /// Returns a value representing the device's state.
    fn value(&self) -> i8 {
        let value = self
            .pin
            .get_value()
            .expect("Could not check if device is active");
        value as i8
    }
}

// ========================================================
// RPPAL
// ========================================================

/// Represents a single device of any type; GPIO-based, SPI-based, I2C-based,
/// etc.  It defines the basic services applicable to all devices
pub trait DeviceR {
    /// Shut down the device and release all associated resources.
    fn close(self);

    /// Returns ``True`` if the device is currently active and ``False`` otherwise.
    fn is_active(&self) -> bool;
}

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
pub struct GpioDeviceR {
    pin: PinR,
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

impl GpioDeviceR {
    /// Returns a GpioDevice with the pin number given
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin which the device is attached to
    ///
    pub fn new(pin: u8) -> GpioDeviceR {
        match Gpio::new() {
            Err(e) => panic!("{:?}", e),
            Ok(gpio) => match gpio.get(pin) {
                Err(e) => panic!("{:?}", e),
                Ok(pin) => GpioDeviceR {
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
