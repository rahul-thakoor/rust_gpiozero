#![crate_type = "lib"]
#![crate_name = "gpiozero"]

//! A simple interface to GPIO devices with Raspberry Pi.
//!
//! This library is based on [GPIOZero](https://gpiozero.readthedocs.io/en/stable/index.html)
//! library.
//!
//! _Note: This is a work in progress. The library will eventually support `embedded-hal` based drivers_
//!
//!
//! The idea is to get started with physical computing using Rust with little coding
//! by hiding the underlying complexity.
//!
//! The library uses [BCM Pin numbering](https://pinout.xyz/)
//!
//! # Example : Blinking an LED
//!
//! ```no_run
//!
//! extern crate gpiozero;
//! use gpiozero::*;
//!
//! fn main() {
//!
//! // Create a new LED attached to Pin 17
//!
//! let mut led = LED::new(17);
//!
//! // blink the LED
//! // on_time: 2 seconds and off_time: 3 seconds
//!
//! led.blink(2,3);
//!
//! }
//! ```
//!
//!
//!  # Example : Wait for a Button Press
//!
//! ```no_run
//!
//! extern crate gpiozero;
//! use gpiozero::*;
//!
//! fn main() {
//! let button = Button::new(17);
//! button.wait_for_press();
//! println!("button pressed");
//!
//! }
//!
//! ```

extern crate sysfs_gpio;

#[cfg(nightly)]
extern crate embedded_hal as hal;

pub use self::output_devices::*;
pub use self::devices::*;
pub use self::traits::*;
pub use self::input_devices::*;
//pub mod led;
pub mod devices;
pub mod output_devices;
pub mod input_devices;
pub mod traits;



