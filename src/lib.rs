#![crate_type = "lib"]
#![crate_name = "gpiozero"]

extern crate sysfs_gpio;

#[cfg(nightly)]
extern crate embedded_hal as hal;


//pub mod led;
pub mod devices;
pub mod output_devices;
pub mod input_devices;
pub mod traits;



