use rust_gpiozero::*;
use std::io;
use std::io::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

static RUNNING: AtomicBool = AtomicBool::new(true);

fn watch_stdin() {
    // wait for key press to exit
    let _ = io::stdin().read(&mut [0u8]).unwrap();
    RUNNING.store(false, Ordering::Relaxed);
}

fn main() {
    std::thread::spawn(watch_stdin);

    // Create a new Servo attached to Pin 23
    let mut servo = Servo::new(23);

    while RUNNING.load(Ordering::Relaxed) {
        servo.max();
        thread::sleep(Duration::from_millis(2_000));
        servo.min();
        thread::sleep(Duration::from_millis(2_000));
    }
}
