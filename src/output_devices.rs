
use devices::GPIODevice;
use sysfs_gpio::{Direction,Pin};
use std::thread;
use std::time::Duration;


#[derive(Debug)]
pub struct OutputDevice {
    pub pin : Pin
}



impl OutputDevice {
    pub fn new(pin:u64) -> OutputDevice{
        let gpiodevice = GPIODevice::new(pin);
        // set direction to output
        gpiodevice.pin.set_direction(Direction::Out).expect("Could not set pin to Output mode");
        OutputDevice {
            pin: gpiodevice.pin
            }
    }


    /* /// Turns the device on.

    pub fn on(&mut self){
        self.pin.set_value(1).expect("Could not turn pin ON");
    }

    /// Turns the device off.
    pub fn off(&mut self){
        self.pin.set_value(0).expect("Could not turn pin OFF");
    }
    
    /// Reverse the state of the device. If it's on, turn it off; if it's off,
    /// turn it on.
    pub fn toggle(&mut self) {
        match self.pin.get_value() {
            Ok(value) => if value == 1 { self.off() } else { self.on() },
            Err(e) => println!("error toggling pin: {:?}", e),
        }
    } */
}

/* /// Represents a generic output device with typical on/off behaviour.
#[derive(Debug)]
pub struct DigitalOutputDevice {
    
   output: OutputDevice,
   pin : Pin

}

impl DigitalOutputDevice {
    fn new(pin:u64) -> DigitalOutputDevice{
        DigitalOutputDevice { output: OutputDevice::new(pin) }
    }

    fn blink(&mut self, on_time:u64, off_time:u64){
        loop {
            self.output.on();
            thread::sleep(Duration::from_secs(on_time));
            self.output.off();
            thread::sleep(Duration::from_secs(off_time));
        }
    }
}

/// Represents a light emitting diode (LED)
#[derive(Debug)]
pub struct LED {
    pub output: DigitalOutputDevice,
    pub pin : Pin
}

impl LED{
    pub fn new(pin:u64) -> LED{
        
       let dout = DigitalOutputDevice::new(pin);
       LED {
            output : dout,
            pin : dout.output.output.gpio.pin
       }
    }

    pub fn blink(&mut self,on_time:u64, off_time:u64){
        self.output.blink(on_time,off_time);
    }

    //TODO: is_lit
} */

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

    pub fn blink(&mut self, on_time:u64, off_time:u64){
        loop {
            self.on();
            thread::sleep(Duration::from_secs(on_time));
            self.off();
            thread::sleep(Duration::from_secs(off_time));
        }
    }
}

impl OutputDeviceTrait for DigitalOutputDevice {

    fn pin(&self) -> Pin {
        self.pin
    }
}

/// supertrait
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
    
   