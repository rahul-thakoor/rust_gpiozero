//! Output device component interfaces for devices such as `LED`
use devices::GPIODevice;
use sysfs_gpio::{Direction,Pin};
use std::thread;
use std::time::Duration;
use traits::*;

/// Represents a generic GPIO output device.
#[derive(Debug)]
pub struct OutputDevice {
    pub pin : Pin
}



impl OutputDevice {
    pub fn new(pin: u64) -> OutputDevice {
        let gpiodevice = GPIODevice::new(pin);
        // set direction to output
        gpiodevice.pin.set_direction(Direction::Out).expect("Could not set pin to Output mode");
        OutputDevice {
            pin: gpiodevice.pin
        }
    }
}


pub trait OutputDeviceTrait{
    /// Get the pin
    fn pin(&self) -> Pin ;

    /// Turns the device on.
    fn on(&mut self){
        let pin = self.pin();
        pin.set_value(1).expect("Could not turn pin ON");
    }

    /// Turns the device off.
    fn off(&mut self){
        let pin = self.pin();
        pin.set_value(0).expect("Could not turn pin OFF");
    }

    /// Reverse the state of the device. If it's on, turn it off; if it's off,
    /// turn it on.
    fn toggle(&mut self) {
        let pin = self.pin();
        match pin.get_value() {
            Ok(value) => if value == 1 { self.off() } else { self.on() },
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
        let value =  self.pin.get_value().expect("Could not check if device is active");
        value as i8
    }
}



/// Represents a generic output device with typical on/off behaviour.
/// Extends behaviour with blink 
#[derive(Debug)]
pub struct DigitalOutputDevice {   
   pin : Pin

}

impl DigitalOutputDevice {
    pub fn new(pin:u64) -> DigitalOutputDevice{
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
    fn blink(&mut self, on_time:u64, off_time:u64){
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
    pin: Pin
}

impl LED{
    pub fn new(pin:u64) -> LED{
        let dout = DigitalOutputDevice::new(pin);
        LED{
             pin: dout.pin
        }
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
    pin: Pin
}

impl Buzzer {
    pub fn new(pin:u64) -> Buzzer{
        let dout = DigitalOutputDevice::new(pin);
        Buzzer{
             pin: dout.pin
        }
    }

    pub fn beep(&mut self, on_time:u64, off_time:u64){
        self.blink(on_time,off_time)
    }
}

impl DigitalOutputDeviceTrait for Buzzer {}
impl OutputDeviceTrait for Buzzer {
    fn pin(&self) -> Pin {
        self.pin
    }
}



struct MotorCompositeDevices {
    forward : OutputDevice,
    backward : OutputDevice
}

// Use type aliasing
type ComponentDevices = MotorCompositeDevices;
///  Represents a generic motor connected
///  to a bi-directional motor driver circuit (i.e. an H-bridge).
///  This is a composite device.
///
pub struct  Motor {
    
    devices : ComponentDevices
    
}

impl Motor {
    /// creates a new Motor instance
    pub fn new(forward_pin:u64, backward_pin:u64) -> Motor{
        let forward = OutputDevice::new(forward_pin);
        let backward = OutputDevice::new(backward_pin);
        let devices = ComponentDevices {forward, backward};
        Motor {
            devices
        }
        
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
   fn close(&self){
       self.devices.forward.close();
       self.devices.backward.close();
   }
}

