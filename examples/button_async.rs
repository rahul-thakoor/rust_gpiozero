//! Display message in console when a Button is pressed
use rust_gpiozero::{Button, Debounce};
use std::time::Duration;

fn main() {
    // Create a button which is attached to Pin 17
    let mut button = Button::new(17)
        // Add debouncing so that subsequent presses within 100ms don't trigger a press
        .debounce(Duration::from_millis(100));

    // Add an async interrupt to trigger whenever the button is pressed
    button
        .when_pressed(|_| {
            println!("button pressed");
        })
        .unwrap();
}
