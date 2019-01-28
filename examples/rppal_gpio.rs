use rust_gpiozero::*;
use std::thread;

use std::time::Duration;
fn main() {
    // Create a button which is attached to Pin 17

    let mut o = OutputDeviceR::new(17);

    o.set_active_high(false);
    o.off();
    println!("{}", o.is_active());
    thread::sleep(Duration::from_secs(3));

    o.on();
    println!("{}", o.is_active());
    thread::sleep(Duration::from_secs(3));
}
