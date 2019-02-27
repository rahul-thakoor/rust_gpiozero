use rust_gpiozero::*;
use std::io;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a new Servo attached to Pin 23
    let mut servo = Servo::new(23);

    loop{
        servo.max();
        thread::sleep(Duration::from_millis(2000));
        servo.min();
        thread::sleep(Duration::from_millis(2000));
    }

    // wait for key press to exit
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
