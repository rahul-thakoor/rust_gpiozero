//! Display message in console when a Button is pressed

use rust_gpiozero::*;


fn main() {
    // Create a button which is attached to Pin 17
    let button = Button::new(17);
    button.wait_for_press();
    println!("button pressed");

}
