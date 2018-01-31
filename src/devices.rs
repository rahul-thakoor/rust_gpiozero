use sysfs_gpio::Pin;

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
    
    fn close(&self){
        if self.pin.is_exported() {
            //TODO implement better error handling
            self.pin.unexport().expect("Could not unexport the selected gpio");
        }
    }
}

