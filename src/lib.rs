#![crate_type = "lib"]
#![crate_name = "gpiozero"]

//! A simple interface to GPIO devices with Raspberry Pi.
//!
//! This library is heavily based on [GPIOZero](https://gpiozero.readthedocs.io/en/stable/index.html)
//! library by Created by [Ben Nuttall](https://github.com/bennuttall) of the `Raspberry Pi Foundation`,
//! [Dave Jones](https://github.com/waveform80), and other contributors
//!
//!
//! The idea is to get started with physical computing using Rust with little coding
//! by hiding the underlying complexity
//! # Example : Blinking an LED
//!
//! ```no_run
//!
//! extern crate gpiozero;
//!
//! use gpiozero::*;
//!
//! fn main() {

//!    let mut led = LED::new(14);
//!    led.blink();
//!
//!  }
//! ```


extern crate sysfs_gpio;

#[cfg(nightly)]
extern crate embedded_hal as hal;


//pub mod led;
pub mod devices;
pub mod output_devices;
pub mod input_devices;
pub mod traits;



