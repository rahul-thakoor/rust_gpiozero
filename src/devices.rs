use sysfs_gpio::Pin;
use traits::Device;

#[derive(Debug)]
pub struct GPIODevice {
    pub pin: Pin
}

impl GPIODevice {
    pub fn new(pin:u64) -> GPIODevice{
        //Create a new Pin with the provided pin_num
        let  gpio = Pin::new(pin);
         //check if pin is not already exported
       
        //try to export the selected pin
        //TODO implement better error handling
        gpio.export().expect("Could not export the selected gpio");
        GPIODevice {pin:gpio}
    }
    
    
}


impl Device for GPIODevice {
    fn pin(&self) -> Pin {
       self.pin
    }

     /// Returns a value representing the device's state.
    fn value(&self) -> i8 {
        let value =  self.pin.get_value().expect("Could not check if device is active");
        value as i8
    }
 
}
