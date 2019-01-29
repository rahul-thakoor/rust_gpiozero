//! Output device component interfaces for devices such as `LED`, `PWMLED`, etc
use rppal::gpio::{Gpio, IoPin, Level, Mode};

/// Represents a generic GPIO output device.
#[derive(Debug)]
pub struct OutputDeviceR {
    pin: IoPin,
    active_state: bool,
    inactive_state: bool,
}

macro_rules! impl_output_device {
    () => {
    /// Set the state for active_high
    pub fn set_active_high(&mut self, value: bool) {
        if value {
            self.active_state=true;
            self.inactive_state=false;
        } else {
            self.active_state=false;
            self.inactive_state=true;
        }
    }
    /// When ``True``, the `value` property is ``True`` when the device's
    /// `pin` is high. When ``False`` the `value` property is
    /// ``True`` when the device's pin is low (i.e. the value is inverted).
    /// Be warned that changing it will invert `value` (i.e. changing this property doesn't change
    /// the device's pin state - it just changes how that state is interpreted).
    pub fn active_high(&self) -> bool {
        self.active_state
    }

    fn value_to_state(&self, value: bool) -> bool {
        if value {
            self.active_state
        } else {
            self.inactive_state
        }
    }

    fn write_state(&mut self, value: bool) {
        if self.value_to_state(value) {
            self.pin.set_high()
        } else {
            self.pin.set_low()
        }
    }

    /// Turns the device on.
    pub fn on(&mut self) {
        self.write_state(true)
    }

    /// Turns the device off.
    pub fn off(&mut self) {
        self.write_state(false)
    }
    /// Reverse the state of the device. If it's on, turn it off; if it's off, turn it on.
    pub fn toggle(&mut self) {
        self.pin.toggle()
    }

    pub fn state_to_value(&self, state: bool) -> bool {
        state == self.active_state
    }

    /// Returns ``True`` if the device is currently active and ``False`` otherwise.
    pub fn value(&self) -> bool {
        match self.pin.read() {
            Level::Low => self.state_to_value(false),
            Level::High => self.state_to_value(true),
        }
    }
    }
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
                    pin: pin.into_io(Mode::Output),
                    active_state: true,
                    inactive_state: false,
                },
            },
        }
    }

    impl_device!();
    impl_gpio_device!();
    impl_output_device!();
}
