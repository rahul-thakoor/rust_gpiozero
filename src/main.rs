extern crate gpiozero;

use std::thread::sleep;
use std::time::Duration;

use gpiozero::led::Led;

fn main() {
    let led = Led::new(17);

    loop {
            led.on();
            sleep(Duration::from_millis(1500));
            led.off();
            sleep(Duration::from_millis(1500));
        }
}
