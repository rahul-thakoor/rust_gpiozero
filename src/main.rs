extern crate gpiozero;

use std::thread::sleep;
use std::time::Duration;

use gpiozero::led::Led;


fn main() {
    let c = Led::new(17);

    loop {
        c.toggle();
        thread::sleep(Duration::from_millis(1000));
    }
}