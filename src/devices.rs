use sysfs_gpio::{Pin, Direction};

#[derive(Debug)]
pub struct GPIODevice {
    pin: Pin
}


impl GPIODevice {
    pub fn new(pin_num:u64) -> GPIODevice{
        //Create a new Pin with the provided pin_num
        let mut gpio = Pin::new(pin_num);
         //check if pin is not already exported
        if gpio.is_exported(){
            println!("Gpio {} is not available", gpio.get_pin());
        }else{
            //try to export the selected pin
            match gpio.export() {
                Ok(()) => println!("Gpio {} exported!", gpio.get_pin()),
                Err(err) => println!("Gpio {} could not be exported: {}", gpio.get_pin(), err),
            }
            return GPIODevice {pin:gpio}
        }
        
    }

    pub close(&self){
        if self.pin.is_exported(){
            match self.pin.unexport() {
            Ok(()) => println!("Gpio {} unexported!", gpio.get_pin()),
            Err(err) => println!("Gpio {} could not be unexported: {}", gpio.get_pin(), err),
            }
        }
         
    }
}

