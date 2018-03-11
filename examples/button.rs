//! Display message in console when a Button is pressed
extern crate gpiozero;
use gpiozero::*;


fn main() {
    // Create a button which is attached to Pin 17
    let button = Button::new(17);
    button.wait_for_press();
    println!("button pressed");

}
