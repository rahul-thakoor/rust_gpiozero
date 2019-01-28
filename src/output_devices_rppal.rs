//! Output device component interfaces for devices such as `LED`, `PWMLED`, etc
use crate::devices::DeviceR;
use rppal::gpio::{Gpio, Level, OutputPin};

/// Represents a generic GPIO output device.
#[derive(Debug)]
pub struct OutputDeviceR {
    pin: OutputPin,
    active_state: bool,
    inactive_state: bool,
    pin_level: Level,
}

impl OutputDeviceR {
    /// Returns an OutputDevice with the pin number given
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin which the device is attached to
    ///  
    pub fn new(pin: u8) -> OutputDeviceR {
        match Gpio::new() {
            Err(e) => panic!("{:?}", e),
            Ok(gpio) => match gpio.get(pin) {
                Err(e) => panic!("{:?}", e),
                Ok(pin) => OutputDeviceR {
                    pin: pin.into_output(),
                    active_state: true,
                    inactive_state: false,
                    pin_level: Level::Low,
                },
            },
        }
    }
}

pub trait OutputDeviceT {
    /// Get the pin
    fn get_pin(&mut self) -> &mut OutputPin;

    /// Set the active_state
    fn set_active_state(&mut self, val: bool);

    /// Get the active_state
    fn get_active_state(&self) -> bool;

    /// Get the inactive_state
    fn get_inactive_state(&self) -> bool;

    /// Set the inactive_state
    fn set_inactive_state(&mut self, val: bool);

    /// Set the pin level
    fn set_pin_level(&mut self, level: Level) {
        self.get_pin().write(level);
        self.set_pin_state(level);
    }

    fn get_pin_level(&self) -> Level;
    fn set_pin_state(&mut self, level: Level);

    /// Set the state for active_high
    fn set_active_high(&mut self, value: bool) {
        if value {
            self.set_active_state(true);
            self.set_inactive_state(false);
        } else {
            self.set_active_state(false);
            self.set_inactive_state(true);
        }
    }
    /// When ``True``, the `value` property is ``True`` when the device's
    /// `pin` is high. When ``False`` the `value` property is
    /// ``True`` when the device's pin is low (i.e. the value is inverted).
    /// Be warned that changing it will invert `value` (i.e. changing this property doesn't change
    /// the device's pin state - it just changes how that state is interpreted).
    fn active_high(&self) -> bool {
        self.get_active_state()
    }

    fn value_to_state(&self, value: bool) -> bool {
        if value {
            self.get_active_state()
        } else {
            self.get_inactive_state()
        }
    }

    fn write_state(&mut self, value: bool) {
        if self.value_to_state(value) {
            self.set_pin_level(Level::High)
        } else {
            self.set_pin_level(Level::Low)
        }
    }

    /// Turns the device on.
    fn on(&mut self) {
        self.write_state(true)
    }

    /// Turns the device off.
    fn off(&mut self) {
        self.write_state(false)
    }

    fn state_to_value(&self, state: bool) -> bool {
        state == self.get_active_state()
    }

    /// Returns ``True`` if the device is currently active and ``False`` otherwise.
    fn value(&self) -> bool {
        match self.get_pin_level() {
            Level::Low => self.state_to_value(false),
            Level::High => self.state_to_value(true),
        }
    }
}

impl OutputDeviceT for OutputDeviceR {
    /// Get the pin
    fn get_pin(&mut self) -> &mut OutputPin {
        &mut self.pin
    }

    /// Set the active_state
    fn set_active_state(&mut self, val: bool) {
        self.active_state = val
    }

    /// Get the active_state
    fn get_active_state(&self) -> bool {
        self.active_state
    }
    /// Get the inactive_state
    fn get_inactive_state(&self) -> bool {
        self.inactive_state
    }

    /// Set the inactive_state
    fn set_inactive_state(&mut self, val: bool) {
        self.inactive_state = val
    }

    fn set_pin_state(&mut self, level: Level) {
        self.pin_level = level
    }

    fn get_pin_level(&self) -> Level {
        self.pin_level
    }
}

impl DeviceR for OutputDeviceR {
    /// Returns ``True`` if the device is currently active and ``False`` otherwise.
    fn is_active(&self) -> bool {
        self.value()
    }

    fn close(self) {
        drop(self)
    }
}
