
use devices::GPIODevice;
use sysfs_gpio::Direction;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct OutputDevice {
    pub gpio: GPIODevice
}



impl OutputDevice {
    pub fn new(pin:u64) -> OutputDevice{
        let gpiodevice = GPIODevice::new(pin);
        // set direction to output
        gpiodevice.pin.set_direction(Direction::Out).expect("Could not set pin to Output mode");
        OutputDevice {gpio:gpiodevice}
    }


    /// Turns the device on.

    pub fn on(&mut self){
        self.gpio.pin.set_value(1).expect("Could not turn pin ON");
    }

    /// Turns the device off.
    pub fn off(&mut self){
        self.gpio.pin.set_value(0).expect("Could not turn pin OFF");
    }
    
    /// Reverse the state of the device. If it's on, turn it off; if it's off,
    /// turn it on.
    pub fn toggle(&mut self) {
        match self.gpio.pin.get_value() {
            Ok(value) => if value == 1 { self.off() } else { self.on() },
            Err(e) => println!("error toggling pin: {:?}", e),
        }
    }
}

/// Represents a generic output device with typical on/off behaviour.
#[derive(Debug)]
pub struct DigitalOutputDevice {
    
   output: OutputDevice

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
    output: DigitalOutputDevice
}

impl LED{
    pub fn new(pin:u64) -> LED{
        LED {output : DigitalOutputDevice::new(pin)
        }
    }

    pub fn blink(&mut self,on_time:u64, off_time:u64){
        self.output.blink(on_time,off_time);
    }

    //TODO: is_lit
}