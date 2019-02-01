#![crate_type = "lib"]
#![crate_name = "rust_gpiozero"]

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
//! extern crate rust_gpiozero;
//! use rust_gpiozero::*;
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
//! extern crate rust_gpiozero;
//! use rust_gpiozero::*;
//!
//! fn main() {
//! let button = Button::new(17);
//! button.wait_for_press();
//! println!("button pressed");
//!
//! }
//!
//! ```

#[cfg(nightly)]
extern crate embedded_hal as hal;

pub use self::devices::*;
pub use self::input_devices::*;
pub use self::output_devices::*;
pub use self::output_devices_rppal::*;
pub use self::input_devices_rppal::*;
pub use self::traits::*;
//pub mod led;
#[macro_use]
pub mod devices;
pub mod input_devices;
pub mod output_devices;
#[macro_use]
pub mod output_devices_rppal;
pub mod input_devices_rppal;
pub mod traits;
