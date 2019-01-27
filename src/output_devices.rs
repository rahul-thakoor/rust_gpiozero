//! Output device component interfaces for devices such as `LED`
use crate::devices::GPIODevice;
use crate::traits::*;
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use std::time::Duration;
use sysfs_gpio::{Direction, Pin};
use wiringpi::pin::{Gpio, SoftPwmPin};

/// Represents a generic GPIO output device.
#[derive(Debug)]
pub struct OutputDevice {
    pub pin: Pin,
}

impl OutputDevice {
    pub fn new(pin: u64) -> OutputDevice {
        let gpiodevice = GPIODevice::new(pin);
        // set direction to output
        gpiodevice
            .pin
            .set_direction(Direction::Out)
            .expect("Could not set pin to Output mode");
        OutputDevice {
            pin: gpiodevice.pin,
        }
    }
}

pub trait OutputDeviceTrait {
    /// Get the pin
    fn pin(&self) -> Pin;

    /// Turns the device on.
    fn on(&mut self) {
        let pin = self.pin();
        pin.set_value(1).expect("Could not turn pin ON");
    }

    /// Turns the device off.
    fn off(&mut self) {
        let pin = self.pin();
        pin.set_value(0).expect("Could not turn pin OFF");
    }

    /// Reverse the state of the device. If it's on, turn it off; if it's off,
    /// turn it on.
    fn toggle(&mut self) {
        let pin = self.pin();
        match pin.get_value() {
            Ok(value) => {
                if value == 1 {
                    self.off()
                } else {
                    self.on()
                }
            }
            Err(e) => println!("error toggling pin: {:?}", e),
        }
    }
}

impl OutputDeviceTrait for OutputDevice {
    fn pin(&self) -> Pin {
        self.pin
    }
}

impl Device for OutputDevice {
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

/// Represents a generic output device with typical on/off behaviour.
/// Extends behaviour with blink
#[derive(Debug)]
pub struct DigitalOutputDevice {
    pin: Pin,
}

impl DigitalOutputDevice {
    pub fn new(pin: u64) -> DigitalOutputDevice {
        let outpin = OutputDevice::new(pin);
        DigitalOutputDevice { pin: outpin.pin }
    }
}

impl OutputDeviceTrait for DigitalOutputDevice {
    fn pin(&self) -> Pin {
        self.pin
    }
}

// supertrait
pub trait DigitalOutputDeviceTrait: OutputDeviceTrait {
    // add code here
    fn blink(&mut self, on_time: u64, off_time: u64) {
        loop {
            self.on();
            thread::sleep(Duration::from_secs(on_time));
            self.off();
            thread::sleep(Duration::from_secs(off_time));
        }
    }
}

///  Represents a light emitting diode (LED)
///
/// # Example
///  Connect LED as shown below, with cathode(short leg) connected to GND
///
/// ```shell
///           Resistor     LED
///  Pin 14 o--/\/\/---->|------o GND
///  ```
///
/// ```no_run
///
/// extern crate rust_gpiozero;
///
/// use rust_gpiozero::*;
///
/// fn main() {

///    let mut led = LED::new(14);
///    led.blink(1,1);
///
///  }
///

#[derive(Debug)]
pub struct LED {
    pin: Pin,
}

impl LED {
    pub fn new(pin: u64) -> LED {
        let dout = DigitalOutputDevice::new(pin);
        LED { pin: dout.pin }
    }
}

impl DigitalOutputDeviceTrait for LED {}
impl OutputDeviceTrait for LED {
    fn pin(&self) -> Pin {
        self.pin
    }
}

/// Extends DigitalOutputDevice and represents a digital buzzer component.
///
/// Connect the cathode (negative pin) of the buzzer to a ground pin;
/// connect the other side to any GPIO pin.

#[derive(Debug)]
pub struct Buzzer {
    pin: Pin,
}

impl Buzzer {
    pub fn new(pin: u64) -> Buzzer {
        let dout = DigitalOutputDevice::new(pin);
        Buzzer { pin: dout.pin }
    }

    pub fn beep(&mut self, on_time: u64, off_time: u64) {
        self.blink(on_time, off_time)
    }
}

impl DigitalOutputDeviceTrait for Buzzer {}
impl OutputDeviceTrait for Buzzer {
    fn pin(&self) -> Pin {
        self.pin
    }
}

struct MotorCompositeDevices {
    forward: OutputDevice,
    backward: OutputDevice,
}

// Use type aliasing
type ComponentDevices = MotorCompositeDevices;
///  Represents a generic motor connected
///  to a bi-directional motor driver circuit (i.e. an H-bridge).
///  This is a composite device.
///
pub struct Motor {
    devices: ComponentDevices,
}

impl Motor {
    /// creates a new Motor instance
    pub fn new(forward_pin: u64, backward_pin: u64) -> Motor {
        let forward = OutputDevice::new(forward_pin);
        let backward = OutputDevice::new(backward_pin);
        let devices = ComponentDevices { forward, backward };
        Motor { devices }
    }

    /// Drive the motor forwards.
    pub fn forward(&mut self) {
        self.devices.backward.off();
        self.devices.forward.on();
    }

    /// Drive the motor backwards.
    pub fn backward(&mut self) {
        self.devices.forward.off();
        self.devices.backward.on();
    }

    /// Stop the motor.
    pub fn stop(&mut self) {
        self.devices.forward.off();
        self.devices.backward.off();
    }
}

impl CompositeDevices for Motor {
    fn close(&self) {
        self.devices.forward.close();
        self.devices.backward.close();
    }
}

/// Generic output device configured for software pulse-width modulation (PWM).
/// The pulse width of the signal will be 100μs with a value range of [0,100] (where 0 is a constant low and 100 is a constant high) resulting in a frequenzy of 100 Hz.
pub struct PWMOutputDevice {
    pin: Arc<Mutex<SoftPwmPin<Gpio>>>,
    blinking: Arc<AtomicBool>,
    value: Arc<AtomicIsize>,
}
impl PWMOutputDevice {
    /// Returns a PWMOutputDevice with the pin number given
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin which the device is attached to
    ///
    pub fn new(pin: u16) -> PWMOutputDevice {
        let pi = wiringpi::setup_gpio();

        PWMOutputDevice {
            pin: Arc::new(Mutex::new(pi.soft_pwm_pin(pin))),
            blinking: Arc::new(AtomicBool::new(false)),
            value: Arc::new(AtomicIsize::new(0)),
        }
    }

    /// Turns the device on.
    pub fn on(&mut self) {
        self.stop();
        self.set_value(1.0)
    }

    /// Turns the device off.
    pub fn off(&mut self) {
        self.stop();
    }

    /// Make the device turn on and off repeatedly
    /// # Arguments
    /// * `on_time` - Number of seconds on
    /// * `off_time` - Number of seconds off
    /// * `fade_in_time` - Number of seconds to spend fading in
    /// * `fade_out_time` - Number of seconds to spend fading out
    /// * `n` - Number of times to blink, None means forever.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_gpiozero::*;
    /// let mut led = PWMOutputDevice::new(17);
    /// // Run 5 times
    /// led.blink(2.0, 2.0, 1.0, 1.0, Some(5))
    /// ```    
    pub fn blink(
        &mut self,
        on_time: f32,
        off_time: f32,
        fade_in_time: f32,
        fade_out_time: f32,
        n: Option<i32>,
    ) {
        self.stop();

        let fps = 25 as f32;
        let sequence: Vec<(f32, f32)> =
            Self::generate_sequence(on_time, off_time, fade_in_time, fade_out_time, fps);

        let pin = Arc::clone(&self.pin);
        let blinking = self.blinking.clone();
        let pin_value = self.value.clone();

        thread::spawn(move || {
            blinking.store(true, Ordering::Relaxed);

            let process = Self::blink_background_process();

            match n {
                Some(end) => {
                    for _ in 0..end {
                        process(&pin, &sequence, &blinking, &pin_value)
                    }
                }
                None => loop {
                    process(&pin, &sequence, &blinking, &pin_value)
                },
            }
        });
    }

    /// Make the device fade in and out repeatedly.
    /// # Arguments
    /// * `fade_in_time` - Number of seconds to spend fading in
    /// * `fade_out_time` - Number of seconds to spend fading out
    /// * `n` - Number of times to pulse; None means forever.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_gpiozero::*;
    /// let mut led = PWMOutputDevice::new(17);
    /// // Run forever
    /// led.pulse(2.0, 2.0, None)
    /// ```    
    pub fn pulse(&mut self, fade_in_time: f32, fade_out_time: f32, n: Option<i32>) {
        self.blink(0.0, 0.0, fade_in_time, fade_out_time, n)
    }

    /// Get the duty cycle of the PWM device. 0.0 is off, 1.0 is fully on.
    /// Values in between specify varying levels of power in the device.
    pub fn value(&self) -> f32 {
        (self.value.load(Ordering::Relaxed)) as f32 / 100.0
    }

    /// Returns True if the device is currently active (value is non-zero) and False otherwise.
    pub fn is_active(&self) -> bool {
        self.value() > 0.0
    }

    /// Set the duty cycle of the PWM device. 0.0 is off, 1.0 is fully on.
    /// Values in between may be specified for varying levels of power in the device.
    pub fn set_value(&self, value: f32) {
        let val = (value * 100.0) as i32;
        self.pin.lock().unwrap().pwm_write(val);
        self.value.store(val as isize, Ordering::SeqCst);
    }

    /// Toggle the state of the device.
    /// If the device is currently off (value is 0.0), this changes it to “fully” on (value is 1.0).
    /// If the device has a duty cycle (value) of 0.1, this will toggle it to 0.9, and so on.
    /// Cannot be used if device is blinking or pulsing
    pub fn toggle(&mut self) {
        if self.blinking.load(Ordering::SeqCst) {
            // Do nothing if background thread is blinking device
            return;
        } else {
            let val = 1.0 - self.value();
            self.set_value(val)
        }
    }

    fn blink_background_process(
    ) -> impl Fn(&Arc<Mutex<SoftPwmPin<Gpio>>>, &Vec<(f32, f32)>, &Arc<AtomicBool>, &Arc<AtomicIsize>)
    {
        |pin, sequence, blinking, pin_value| {
            let pin = pin.lock().unwrap();

            for (value, delay) in sequence {
                if !blinking.load(Ordering::SeqCst) {
                    pin.pwm_write(0);
                    pin_value.store(0, Ordering::SeqCst);
                    break;
                }
                pin.pwm_write((value * 100 as f32) as i32);
                pin_value.store((value * 100 as f32) as isize, Ordering::Relaxed);
                thread::sleep(Duration::from_millis((delay * 1000 as f32) as u64));
            }
        }
    }

    fn generate_sequence(
        on_time: f32,
        off_time: f32,
        fade_in_time: f32,
        fade_out_time: f32,
        fps: f32,
    ) -> Vec<(f32, f32)> {
        let mut sequence: Vec<(f32, f32)> = Vec::new();

        // create sequence for fading in
        for i in 0..fps as i32 * fade_in_time as i32 {
            sequence.push((i as f32 * (1.0 / fps) / fade_in_time, 1.0 / fps))
        }

        // allow to stay on for on_time
        sequence.push((1.0, on_time));

        // create sequence for fading out
        for i in 0..fps as i32 * fade_out_time as i32 {
            sequence.push((1.0 - (i as f32 * (1.0 / fps) / fade_out_time), 1.0 / fps))
        }

        // allow to stay off for off_time
        sequence.push((0.0, off_time));

        sequence
    }

    fn stop(&mut self) {
        self.blinking.clone().store(false, Ordering::SeqCst);
        self.set_value(0.0)
    }
}

/// Represents a light emitting diode (LED) with variable brightness.
/// A typical configuration of such a device is to connect a GPIO pin
/// to the anode (long leg) of the LED, and the cathode (short leg) to ground,
/// with an optional resistor to prevent the LED from burning out.
pub struct PWMLED(PWMOutputDevice);

impl PWMLED {
    /// Returns a PMWLED with the pin number given
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin which the device is attached to
    ///    
    pub fn new(pin: u16) -> PWMLED {
        PWMLED(PWMOutputDevice::new(pin))
    }

    /// Returns True if the device is currently active (value is non-zero) and False otherwise.
    pub fn is_lit(&self) -> bool {
        self.0.is_active()
    }
    /// Make the device turn on and off repeatedly
    /// # Arguments
    /// * `on_time` - Number of seconds on
    /// * `off_time` - Number of seconds off
    /// * `fade_in_time` - Number of seconds to spend fading in
    /// * `fade_out_time` - Number of seconds to spend fading out
    /// * `n` - Number of times to blink, None means forever.
    ///
    pub fn blink(
        &mut self,
        on_time: f32,
        off_time: f32,
        fade_in_time: f32,
        fade_out_time: f32,
        n: Option<i32>,
    ) {
        self.0
            .blink(on_time, off_time, fade_in_time, fade_out_time, n)
    }

    /// Turns the device on.
    pub fn on(&mut self) {
        self.0.on();
    }

    /// Turns the device off.
    pub fn off(&mut self) {
        self.0.off();
    }

    /// Make the device fade in and out repeatedly.
    /// # Arguments
    /// * `fade_in_time` - Number of seconds to spend fading in
    /// * `fade_out_time` - Number of seconds to spend fading out
    /// * `n` - Number of times to pulse; None means forever.
    ///
    pub fn pulse(&mut self, fade_in_time: f32, fade_out_time: f32, n: Option<i32>) {
        self.0.pulse(fade_in_time, fade_out_time, n);
    }

    /// Toggle the state of the device.
    /// If the device is currently off (value is 0.0), this changes it to “fully” on (value is 1.0).
    /// If the device has a duty cycle (value) of 0.1, this will toggle it to 0.9, and so on.
    /// Cannot be used if device is blinking or pulsing
    pub fn toggle(&mut self) {
        self.0.toggle();
    }

    /// Set the duty cycle of the PWM device. 0.0 is off, 1.0 is fully on.
    /// Values in between may be specified for varying levels of power in the device.
    pub fn set_value(&self, value: f32) {
        self.0.set_value(value);
    }

    /// Get the duty cycle of the PWM device. 0.0 is off, 1.0 is fully on.
    /// Values in between specify varying levels of power in the device.
    pub fn value(&self) -> f32 {
        self.0.value()
    }
}
