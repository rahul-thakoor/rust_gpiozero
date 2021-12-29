# rust_gpiozero

[![CI](https://github.com/rahul-thakoor/rust_gpiozero/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/rahul-thakoor/rust_gpiozero/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/rust_gpiozero)](https://crates.io/crates/rust_gpiozero)

A simple interface to GPIO devices with Raspberry Pi.

This library is based on [GPIOZero](https://gpiozero.readthedocs.io/en/stable/index.html)
library.


The idea is to get started with physical computing using Rust with little coding
by hiding the underlying complexity.

The library uses [BCM Pin numbering](https://pinout.xyz/)

### Example : Blinking an LED

```rust

use rust_gpiozero::*;

fn main() {
    // Create a new LED attached to Pin 17
    let mut led = LED::new(17);

    // on_time = 2 secs, off_time=3 secs
    led.blink(2.0,3.0);

    // prevent program from exiting immediately
    led.wait();
}

```


### Example : Wait for a Button Press
```rust

use rust_gpiozero::*;

fn main() {
    // Create a button which is attached to Pin 17
    let mut button = Button::new(17);
    button.wait_for_press(None);
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

To use `rust_gpiozero`, first add this to your Cargo.toml:

```toml
[dependencies]
 rust_gpiozero = "0.2.0"
```
Compiling your project on a Raspberry Pi directly can take significant time depending on the model. Ideally, you would cross compile your project then run it on the Raspberry Pi. 

[More information](https://github.com/japaric/rust-cross)

## Features

The following features are planned :

- [ ] Support for `embedded-hal`
- [ ] Support for common devices such as Accelerometer, Temperature sensors, etc

## Changelog

[CHANGELOG.md](https://github.com/rahul-thakoor/rust_gpiozero/blob/master/CHANGELOG.md)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

Thanks for your interest in `rust_gpiozero`. I am a newbie rustacean and just started using the language! I am using this project to learn more about Rust. Feel free to give feedback or send PRs. Your experiences and feedback will also benefit others who use this library.

## Credits
This library would not be possible without the great work of the maintainers of [GPIOZero](https://gpiozero.readthedocs.io/en/stable/index.html) and [rppal](https://github.com/golemparts/rppal)






































































