use sysfs_gpio::Pin;
use sysfs_gpio::Edge;

/// Represents a single device of any type; GPIO-based, SPI-based, I2C-based,
/// etc.  It defines the basic services applicable to all devices

pub trait Device {
    /// Get the pin
    fn pin(&self) -> Pin ;

    /// Shut down the device and release all associated resources.
    fn close(&self){
        let pin = self.pin();
        if pin.is_exported() {
            //TODO implement better error handling
            pin.unexport().expect("Could not close device");
        }
    }

    /// Returns a value representing the device's state.
    fn value(&self) -> i8;


    /// Returns ``True`` if the device is currently active and ``False``otherwise.
    fn is_active(&self) -> bool{
        let value =  self.value();
        if value >= 1 { true } else {false }
    }
}


/// Adds edge-detected `when_activated` and `when_deactivated`
/// events to a device based on changes to the `is_active`
/// property common to all devices.
pub trait EventsTrait:Device {
    fn pin(&self) -> Pin ;
    /// Pause the program until the device is activated
    fn wait_for_active(&self) {
        
        loop {
            if self.is_active() == true {break;}
        }

    }

    /// Pause the program until the device is deactivated
    fn wait_for_inactive(&self) {
        
        loop {
            if self.is_active() == false {break;}
        }

    }

    
}