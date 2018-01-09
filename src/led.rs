
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, Mode, Level};

#[allow(dead_code)]
pub struct Led {
    pin: u8,
    gpio : Gpio
}

impl Led {

    pub fn new(pin:u8) -> Led{
        
        let mut led_gpio = Gpio::new().unwrap();
        led_gpio.set_mode(pin, Mode::Output);
        Led{
            pin : pin,
            gpio: led_gpio
        }

    }

    pub fn on(&self){
        self.gpio.write(self.pin, Level::High);
    }

    pub fn off(&self){
        self.gpio.write(self.pin, Level::Low)
    }

    pub fn blink(&self, millis:u64){
        loop{
                // Blink an LED attached to the pin on and off
                println!("turning on...");
                self.on();
                thread::sleep(Duration::from_millis(millis));
                println!("turning off...");
                self.off();
                thread::sleep(Duration::from_millis(millis));
                
            }
    }
}