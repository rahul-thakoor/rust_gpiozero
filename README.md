# rust_gpiozero


A simple interface to GPIO devices with Raspberry Pi.

This library is based on [GPIOZero](https://gpiozero.readthedocs.io/en/stable/index.html)
library created by [Ben Nuttall](https://github.com/bennuttall) of the `Raspberry Pi Foundation`,
[Dave Jones](https://github.com/waveform80), and other contributors.

_Note: This is a work in progress. The library will eventually support `embedded-hal` based drivers_


The idea is to get started with physical computing using Rust with little coding
by hiding the underlying complexity.

### Example : Blinking an LED

```rust

extern crate gpiozero;
use gpiozero::*;

fn main() {

// Create a new LED attached to Pin 17

let mut led = LED::new(17);

// blink the LED
// on_time: 2 seconds and off_time: 3 seconds

led.blink(2,3);

}

```


### Example : Wait for a Button Press
```rust
extern crate gpiozero;
use gpiozero::*;


fn main() {
    // Create a button which is attached to Pin 17
    let button = Button::new(17);
    button.wait_for_press();
    println!("button pressed");

}

```


Compare this to using the crate `sysfs_gpio` to blink an LED on the Raspberry Pi :

```rust

extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let my_led = Pin::new(127); // number depends on chip, etc.
    my_led.with_exported(|| {
        loop {
            my_led.set_value(0).unwrap();
            sleep(Duration::from_millis(200));
            my_led.set_value(1).unwrap();
            sleep(Duration::from_millis(200));
        }
    }).unwrap();
}

```


## Install/Use

To use `gpiozero`, first add this to your Cargo.toml:

```toml
[dependencies]
gpiozero = "0.1.0"
```


## Features

The following features are planned :

- [ ] Support for `linux-embedded-hal`
- [ ] Support for common devices such as Accelerometer, Temperature sensors, etc













































































