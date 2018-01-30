#![crate_type = "lib"]
#![crate_name = "gpiozero"]

extern crate sysfs_gpio;

use sysfs_gpio::*;

//pub mod led;
pub mod devices;
